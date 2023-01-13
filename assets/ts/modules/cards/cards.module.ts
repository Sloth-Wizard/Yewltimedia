import { getHtml, getKData } from '../../tools/fetchers';
import card from './templates/card.html';

// Init our files that are inside the data folder
const FILES = [
    'ali',
    'marc',
    'alexis'
];

export default new Promise(async $export => {
    $export(
        class Cards {
            /**
             * ## Build the cards from k files
             * 
             * @param container - HTMLElement
             * 
             * @returns `Promise<void>`
             */
            async create(container: HTMLElement): Promise<void> {
                getHtml(card).then(html => {
                    for (const FILE of FILES) {
                        getKData<{SPRITE?: string; NAME?: string}>(FILE).then(async ({SPRITE, NAME}) => {
                            if (SPRITE && NAME) {
                                const HTML = 
                                    html.replace(
                                        /{o_o}NAME{o_o}/gm,
                                        NAME.toLowerCase()
                                    ).replace(
                                        /{o_o}SPRITE{o_o}/gm, 
                                        `./dist/img/${SPRITE}`
                                    );
                                container.insertAdjacentHTML('beforeend', HTML);
                                const INJECTEDCARD = container.querySelector<HTMLElement>('.card');
                                if (INJECTEDCARD)
                                    INJECTEDCARD.classList.add('active');
                            }
                        });
                    }
                });
            }
        }
    );
});
