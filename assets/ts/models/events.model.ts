export interface ObservedEvent<T> {
    element?: HTMLElement | T;
    event?: Event | SubmitEvent;
    time?: number;
    target?: EventTarget | null;
    error?: any;
}