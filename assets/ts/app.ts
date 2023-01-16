import { Lazyload } from './models/signatures.model';
import importer from './modules/importer';

// import multiple
export default new Promise<{
    name: string;
    lazyload: Lazyload;
}>(async $export => {
    // Grab modules needed after first page load
    const [
        lazyload,
    ] = await importer(
        import('./modules/lazyload/noio.module'),
    );

    $export({ name: 'the-game', lazyload});
}).then(async m => {
    document.body.classList.add('loaded');

    window.__lazyload = new m.lazyload();
    window.__lazyload.load('.js-lazy:not(.b-loaded)');
});
