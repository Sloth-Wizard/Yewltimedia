export interface IexportModule<T> {
    name: string;
    origin: HTMLElement;
    module: T;
}