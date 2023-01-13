export default new Promise(async $export => {
    $export(
        class Sound {
            private soundsFolder: string; // start and end with `/`
            constructor() {
                this.soundsFolder = './dist/sounds/';
            }

            /**
             * ## Start a sound from the kData file
             * 
             * @param kDataSound - string
             * @param sound - HTMLAudioElement|undefined
             * 
             * @returns Promise<HTMLAudioElement>
             */
            async start(kDataSound: string, sound?: HTMLAudioElement): Promise<HTMLAudioElement> {
                if (!sound) {
                    const SOUND = new Audio(`${this.soundsFolder}${kDataSound}`);
                    SOUND.load();
                    SOUND.volume = 0.2;
                    SOUND.play();

                    return SOUND;
                }
                
                sound.pause();
                sound.src = `${this.soundsFolder}${kDataSound}`;
                sound.load();
                sound.play();

                return sound;
            }
        }
    );
});