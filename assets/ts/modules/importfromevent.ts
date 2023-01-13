import { Subscription } from 'rxjs';
import { IexportModule } from '../models/iexportmodule.model';
import { EventsProp } from '../models/signatures.model';

export default async<T>(
    elements: NodeListOf<HTMLElement>, 
    event: string, 
    eventHandler: EventsProp,
    preventDefault: boolean, 
    stopPropagation: boolean, 
    id: string,
    callback: (module: IexportModule<T>) => any
) => {
    let subs: Subscription[] = [];
    for (const EL of elements) {
        EL.dataset.tevt = event;
        subs.push(eventHandler.observe(EL, atob(event), async ev => {
            if (!ev.element)
                return console.error('Stop right there criminal! Loading a script illegally');
            
            new Promise<IexportModule<T>>(async $export => {
                let m: T | undefined;

                if (id === 'tapped')
                    m = await (await import('./tapped/tapped.module')).default as T;

                if (m)
                    $export({name: 'onev-exp', origin: ev.element!, module: m});
                else
                    return false;
            }).then(m => {
                for (const SUB of subs) { SUB.unsubscribe(); }
                for (const EL of elements) { delete EL.dataset.tevt; }

                callback(m);
            });
        }, preventDefault, stopPropagation));
    }
}
