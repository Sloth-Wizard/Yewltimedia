import { EventsProp } from '../../models/signatures.model';
import { getHtml, getKData } from '../../tools/fetchers';
import cardDescription from './templates/cardDescription.html';
import { Subscription } from 'rxjs';
import { Isound } from '../../models/sounds.model';

export default new Promise(async $export => {
    $export(
        class Tapped {
            protected elLastTapTime: Array<number>;
            protected animPlayTimeInMs: number; // To specify a minimum time before replaying the animation
            protected animClassName: string;
            protected animStopClassName: string;
            protected spriteClassName: string;
            protected soundModuleLoaded: boolean;
            protected soundModule: Isound;
            protected audioPlayer: HTMLAudioElement;
            constructor(
                private elements: NodeListOf<HTMLElement>,
                private observer: EventsProp
            ) {
                this.animPlayTimeInMs = 510;
                this.animClassName = 'tapped';
                this.animStopClassName = 'untapped';
                this.spriteClassName = 'sprite';
                this.soundModuleLoaded = false;
            }

            /**
             * ## Start our tap listeners
             * 
             * @param event - string
             * 
             * @returns `Promise<void>`
             */
            async create(event: string): Promise<void> {
                if (!this.soundModuleLoaded) {
                    this.soundModule = new (await (await import('../sounds/sound.module')).default as Isound);
                    this.soundModuleLoaded = true;
                }

                this.elLastTapTime = [];
                for (let i = 0; i < this.elements.length; i++) {
                    this.elLastTapTime[i] = 0;
                    this.elements[i].dataset.listener = `${event}${btoa('<o_o<the_game')}`;
                    this.observer.observe(this.elements[i], atob(event), async ev => {
                        // Check time to avoid triggering another animation before the running one is still playing
                        // Anim time => 500ms
                        // So we check `elLastTapTime` has a difference of at least 501ms to the `ev.time` (add 1ms to let some breathing time)
                        // Dont't forget to double the value as the animation needs to come down too
                        if (
                            this.elLastTapTime[i] === 0 || 
                            ev.time! >= this.elLastTapTime[i] + (this.animPlayTimeInMs * 2) || 
                            (
                                ev.element!.classList.contains('more') &&
                                ev.element!.classList.contains(this.animClassName)
                            )
                        ) {
                            if (ev.element!.classList.contains('more'))
                                this.showMore(ev.element!, event);
                            else
                                this.play(ev.element!);

                            this.elLastTapTime[i] = ev.time!;
                        } else {
                            this.spam();
                        }
                    });
                }
            }

            /**
             * ## Play the animation
             * 
             * @param el - HTMLElement
             * 
             * @returns `Promise<void>`
             */
            private async play(el: HTMLElement): Promise<void> {
                el.classList.add(this.animClassName);
                el.classList.add(this.spriteClassName);

                this.playSoundIfSound(el);

                setTimeout(() => {
                    el.classList.remove(this.animClassName);
                }, this.animPlayTimeInMs);
                setTimeout(() => {
                    el.classList.remove(this.spriteClassName);
                }, this.animPlayTimeInMs * 2);
            }

            /**
             * ## Show the description block
             * 
             * @param el - HTMLElement
             * 
             * @returns 
             */
            private async showMore(el: HTMLElement, event: string) {
                // Data needed for hide of display signs
                const CARDS = Array.from(document.querySelectorAll<HTMLElement>('.card'));
                const ACTIVECARD = el.closest<HTMLElement>('.card');
                const ICON = el.querySelector<HTMLElement>('.icon');
                
                if (el.dataset.nxtcl === '<o_o<') {
                    el.dataset.nxtcl = '>o_o>';
                    el.classList.remove(this.animClassName);
                    
                    // Display + signs of unfocused cards
                    // We wait the main animation to finish before changing back ou zIndex
                    return setTimeout(() => {
                        CARDS.find(card => {
                            if (ACTIVECARD !== card)
                                card.style.zIndex = 'auto';
                        });

                        // Display the + sign of focused card
                        if (ICON)
                            ICON.style.opacity = '1';
                    }, this.animPlayTimeInMs);
                }

                if (!el.dataset.nxtcl || el.dataset.nxtcl === '>o_o>') {
                    el.dataset.nxtcl = '<o_o<';

                    // Hide + signs of unfocused cards
                    CARDS.find(card => {
                        if (ACTIVECARD !== card)
                            card.style.zIndex = '0';
                    });
                    // Hide + sign of focused card
                    if (ICON)
                        ICON.style.opacity = '0';

                    const DESCRIPTIONCONTAINER = document.querySelector<HTMLElement>('._czd');
                    if (DESCRIPTIONCONTAINER) {
                        getHtml(cardDescription).then(html => {
                            getKData<{
                                PICTURE: string;
                                NAME: string;
                                DESCRIPTION: string;
                            }>(el.dataset.k!).then(data => {
                                // Replace the placeholders in the html with the fetched data
                                for (const [K, V] of Object.entries(data)) {
                                    const RGXP = new RegExp(`{o_o}${K}{o_o}`, 'gm')
                                    html = html.replace(RGXP, V);
                                }

                                // Inject the moded html
                                DESCRIPTIONCONTAINER.innerHTML = html;
                                
                                // Replace the image from our data-src on the fly
                                window.__lazyload.load('.js-lazy:not(.b-loaded)');

                                setTimeout(() => {
                                    if (el.classList.contains(this.animClassName)) {
                                        DESCRIPTIONCONTAINER.classList.add(this.animClassName);
                                        // When clicking on the text, remove the class and dispatch that same event to the background
                                        const SUB = this.observer.observe(DESCRIPTIONCONTAINER, atob(event), async ev => {
                                            if (!(ev.target instanceof HTMLAnchorElement))
                                                this.closeShowMore(ev.element!, el, SUB, event);
                                        }, true, true);
                                    }
                                }, this.animPlayTimeInMs);
                            });
                        });
                    }

                    return el.classList.add(this.animClassName);
                }
            }

            /**
             * ## Handle the sound module
             * 
             * @param el - HTMLElement
             * 
             * @returns `Promise<void>`
             */
            private async playSoundIfSound(el: HTMLElement): Promise<void> {
                if (!this.soundModule)
                    throw new Error('Could not load sound module');

                if (this.soundModule && el.dataset.k) {
                    console.log(this.soundModule, el.dataset.k);
                    if (el.dataset.kSound) { // Sound was already loaded
                        this.audioPlayer = await this.soundModule.start(el.dataset.kSound, this.audioPlayer);
                    } else { // Sound needs to be loaded
                        getKData<{
                            SOUND: string;
                        }>(el.dataset.k).then(async data => {
                            if (!el.dataset.kSound)
                                el.dataset.kSound = data.SOUND;

                            this.audioPlayer = await this.soundModule.start(data.SOUND, this.audioPlayer);
                        }).catch(e => {
                            throw new Error(`
                                No sound found inside the kData file\n
                                ==> ${e.message}
                            `);
                        });
                    }
                }
            }

            /**
             * ## Close the card description
             * 
             * @param wrapper - HTMLElement
             * @param origin - HTMLElement
             * @param sub - Subscription
             * @param event - string
             * 
             * @returns `Promise<void>`
             */
            private async closeShowMore(wrapper: HTMLElement, origin: HTMLElement, sub: Subscription, event: string): Promise<void> {
                sub.unsubscribe();
                wrapper.classList.remove(this.animClassName);
                setTimeout(async () => {
                    origin.dispatchEvent(await this.observer.emit(atob(event), {cancelable: true}));
                }, this.animPlayTimeInMs);
            }

            /**
             * ## Pop a too much spam message
             */
            private async spam() {
                console.log('Pourquoi tu spam ?');
            }
        }
    );
});
