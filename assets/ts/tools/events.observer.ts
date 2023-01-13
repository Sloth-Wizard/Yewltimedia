import { fromEvent, MonoTypeOperatorFunction, Observable, Subscription } from 'rxjs';
import { map, tap } from 'rxjs/operators';
import { ObservedEvent } from '../models/events.model';

export default new Promise(async $export => {
    $export(
        class Events {
            constructor() {}
        
            /**
             * ## Observe the passed HTMLElement
             * 
             * The callback param is optional for easier dev purpose but it should always be set in production
             * 
             * ### Usage
             * ```ts
             * let subscription: Subscription = this.observe(el, 'click', async ev => {
             *      // Your logic on event trigger
             * }, false); // Pass to true to trigger a preventDefault
             * ```
             * 
             * @param element - HTMLElement
             * @param event_name - string
             * @param callback - (ev: ObservedEvent) => any
             * @param prevent_default - boolean
             * 
             * @returns `Subscription`
             */
            observe<T extends HTMLElement | Window>(
                element: HTMLElement | T, 
                event_name: string, 
                callback?: (ev: ObservedEvent<T>) => any, 
                prevent_default: boolean = false,
                stop_propagation: boolean = false
            ): Subscription {
                const EVENT: Observable<ObservedEvent<T>> = fromEvent(
                    element,
                    event_name
                ).pipe(
                    this.custom_prevent_default(prevent_default, stop_propagation),
                    map(event => {
                        if (!element || !event.timeStamp || !event) {
                            console.error('Event or element doesn\'t exist somehow');
                            return {
                                error: 'Event or element doesn\'t exist somehow'
                            };
                        }
        
                        return {
                            element: element,
                            event: event,
                            time: event.timeStamp,
                            target: event.target
                        };
                    })
                );
        
                if (!callback)
                    return EVENT.subscribe();
        
                return EVENT.subscribe(callback);
            }
        
            /**
             * ## Custom prevent default
             * 
             * Make it possible to prevent default on a combination of event inside the observer
             * 
             * @param prevent_default - boolean
             * @param stop_propagation - boolean
             * 
             * @returns `MonoTypeOperatorFunction<T>`
             */
            custom_prevent_default<T extends Event>(prevent_default: boolean, stop_propagation: boolean): MonoTypeOperatorFunction<T> {
                return tap(e => {
                    if (prevent_default)
                        e.preventDefault();
        
                    if (stop_propagation)
                        e.stopPropagation();
                });
            }
        
            /**
             * ## Emit an event
             * 
             * @param event_name - string
             * @param event_init - EventInit | undefined
             * 
             * @returns `Promise<Event>`
             */
            async emit<T extends EventInit>(event_name: string, event_init?: T | undefined): Promise<Event> {
                return new Event(event_name, event_init);
            }
        }
    );
});
