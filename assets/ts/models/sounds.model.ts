export interface Isound {
    new();
    start(kDataSound: string, sound?: HTMLAudioElement): Promise<HTMLAudioElement>;
}
