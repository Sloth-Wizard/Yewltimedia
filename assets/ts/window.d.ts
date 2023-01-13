import { LazyloadI } from './models/signatures.model';

export {};

declare global {
    interface Window {
        __lazyload: LazyloadI;
    }
}