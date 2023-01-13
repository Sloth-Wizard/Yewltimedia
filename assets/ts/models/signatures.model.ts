export interface GenericMultiSig<T> {
    new(...args: any): T;
}

export interface GenericCreateSig {
    create(...args: any): Promise<void>;
}

export interface StandardSingleSig {
    new(s: string): SingleS;
}

interface SingleS {
    selector: string;
}

import { ObservedEvent } from './events.model';
import { Subscription, MonoTypeOperatorFunction } from 'rxjs';

export interface EventsSig {
    new(): EventsProp;
}

export interface EventsProp {
    observe<T extends HTMLElement | Window>(
        element: HTMLElement | T,
        event_name: string,
        callback?: (ev: ObservedEvent<T>) => any,
        prevent_default?: boolean,
        stop_propagation?: boolean
    ): Subscription;
    emit<F extends EventInit>(event_name: string, event_init?: F): Promise<Event>;
    custom_prevent_default<G extends Event>(): MonoTypeOperatorFunction<G>;
}


export interface Lazyload {
    new(): LazyloadI;
}

export type LazyloadI = & {
    load(selector: string): Promise<void>
}