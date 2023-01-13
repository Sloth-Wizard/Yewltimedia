import { IexportModule } from './models/iexportmodule.model';
import { EventsSig, GenericCreateSig, GenericMultiSig, Lazyload } from './models/signatures.model';
import importer from './modules/importer';
import importFromEvents from './modules/importfromevent';

// import multiple
export default new Promise<{
    name: string;
    events: EventsSig;
    lazyload: Lazyload;
    importFromEvents: typeof importFromEvents;
    //cards: GenericMultiSig<GenericCreateSig>;
}>(async $export => {
    // Grab modules needed after first page load
    const [
        events,
        lazyload,
        importFromEvents
        //cards
    ] = await importer(
        import('./tools/events.observer'),
        import('./modules/lazyload/noio.module'),
        import('./modules/importfromevent'),
        //import('./modules/cards/cards.module')
    );

    $export({ name: 'the-game', events, lazyload, importFromEvents/*, cards*/});
}).then(async m => {
    document.body.classList.add('loaded');

    const OBSERVER = new m.events();
    const IMPORTFROMEVENTS = m.importFromEvents;

    // Init the cards
    /* This is juste a test to make content load only when it's ready
    const WRAPPER = document.querySelector<HTMLElement>('.card--wrapper');
    if (WRAPPER)
        new m.cards().create(WRAPPER);
    */

    // Load the tapped module on the first tap
    const TAPEVENT = btoa('mousedown');
    const TAPPABLEELEMENTS = document.querySelectorAll<HTMLElement>(`.js-tap:not([data-tevt="${TAPEVENT}"])`);
    IMPORTFROMEVENTS<GenericMultiSig<GenericCreateSig>>(TAPPABLEELEMENTS, TAPEVENT, OBSERVER, false, false, 'tapped', tapped => {
        handleModule(tapped, [TAPPABLEELEMENTS, OBSERVER], TAPEVENT);
    });

    async function handleModule(pmdl: IexportModule<GenericMultiSig<GenericCreateSig>>, args: Array<any>, event: string) {
        if (!pmdl.module)
            return console.error(`Could not import module ${pmdl.name}`);

        const PMDL = new pmdl.module(...args);
        PMDL.create(event).then(async () => {
            pmdl.origin.dispatchEvent(await OBSERVER.emit(atob(event), {cancelable: true}));
        });
    }

    window.__lazyload = new m.lazyload();
    window.__lazyload.load('.js-lazy:not(.b-loaded)');
});
