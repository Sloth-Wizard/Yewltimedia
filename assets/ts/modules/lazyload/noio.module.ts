export default new Promise(async $export => {
    $export(
        class Noio {
            protected images: HTMLImageElement[];
            protected custom_event: Event;
            constructor() {
                this.custom_event = new CustomEvent('__NOIO__', {cancelable: true});
            }

            /**
             * ## Get the images to lazyload
             * 
             * @returns `HTMLImageElement[]`
             */
            private async get_images(selector: string): Promise<HTMLImageElement[]> {
                return Array.from(document.querySelectorAll<HTMLImageElement>(selector))
            }
        
            /**
             * ## Start loading the remaining images
             * 
             * @param selector - string
             * 
             * @returns `Promise<void>`
             */
            async load(selector: string): Promise<void> {
                this.get_images(selector).then(images => {
                    this.images = images;
                    const TRIGGER = async () => {
                        if (this.images.length <= 0) {
                            window.removeEventListener('scroll', TRIGGER);
                            window.removeEventListener('resize', TRIGGER);
                        }
            
                        for (const IMAGE of this.images) {
                            IMAGE.dataset.noiolistener = 'true';
                            const BCR = IMAGE.getBoundingClientRect();
                            if (BCR.top < window.innerHeight && window.getComputedStyle(IMAGE).display !== 'none') {
                                new Promise(() => {
                                    const TEMP_IMAGE = new Image();
                                    TEMP_IMAGE.onload = () => {
                                        IMAGE.src = TEMP_IMAGE.src;
                                        delete IMAGE.dataset.src;
            
                                        IMAGE.classList.add('b-loaded');
            
                                        const FIGURE = IMAGE.closest('figure');
                                        if (FIGURE)
                                            FIGURE.classList.add('img-is-loaded');
                                        
                                        // Remove the temporary created node
                                        TEMP_IMAGE.remove();

                                        // Send a custom event
                                        IMAGE.dispatchEvent(this.custom_event);
                                    }
            
                                    if (IMAGE.dataset.src)
                                        TEMP_IMAGE.src = IMAGE.dataset.src;
            
                                    if (!IMAGE.dataset.src) {
                                        const INDEX = this.images.indexOf(IMAGE);
                                        // Remove the freshly loaded image from the array
                                        this.images.splice(INDEX, 1);
                                    }
                                    
                                });
                            }
                        }  
                    };
            
                    TRIGGER();

                    window.addEventListener('scroll', TRIGGER);
                    window.addEventListener('resize', TRIGGER);
                });
            }
        }
    );
});
