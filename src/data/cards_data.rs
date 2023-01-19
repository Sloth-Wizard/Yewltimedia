use crate::models::cards::Card;

pub fn create_cards() -> Vec<Card> {
    return vec![
        Card {
            picture: "Ali-mini.jpg".to_string(),
            sprite: "Ali.png".to_string(),
            name: "Ali".to_string(),
            description: "<p><i>\"NO comment\"</i></p><strong>Tu fais quoi de tes journées&nbsp;?</strong><p>Inspirer-Expirer.</p><strong>Une passion&nbsp;?</strong><p>La cuisine.</p><strong>À la maison il y a&nbsp;?</strong><p>La paix.</p><strong>Un pseudo&nbsp;?</strong><p>Namilusila.</p><strong>Un dicton/mantra&nbsp;?</strong><p>En attendant Godot.</p><strong>Un film préféré&nbsp;?</strong><p><a href=\"https://www.youtube.com/watch?v=u_s-4OAxkKw\" target=\"_blank\">Cinema Paradiso</a></p><strong>Une musique préférée&nbsp;?</strong><p>La musique classique.</p><strong>Ta BD préférée&nbsp;?</strong><p>Maus - Art Spiegelman.</p><strong>Ce que tu aimes chez Dupuis&nbsp;?</strong><p><a href=\"https://www.dupuis.com/label/FR/1/aire_libre.html\" target=\"_blank\">Aire Libre.</a></p><strong>Quelle est ta bonne résolution pour 2023&nbsp;?</strong><p>Visiter Damas.</p>".to_string(),
            sound: "ali.mp3".to_string()
        },
        Card {
            picture: "Caroline-mini.jpg".to_string(),
            sprite: "Caroline.png".to_string(),
            name: "Caroline".to_string(),
            description: "<p><i>\"Fait un peu tout, même le café&nbsp;!\"</i></p><strong>Tu fais quoi de tes journées&nbsp;?</strong><p>Je surveille mon équipe histoire qu'ils ne fassent pas (trop) de bêtises.</p><strong>Une passion&nbsp;?</strong><p>Stephen King, Maxime Chattam, Éric Giacometti, Franck Thilliez...</p><strong>À la maison il y a&nbsp;?</strong><p>Pluche 🐈 C'est elle qui fait la loi&nbsp;!</p><strong>Un pseudo&nbsp;?</strong><p>@Carochou</p><strong>Un dicton/mantra&nbsp;?</strong><p>N'abandonne pas ton rêve, continue à dormir.</p><strong>Un film préféré&nbsp;?</strong><p><a href=\"https://youtu.be/CKgXGTS9rQQ\" target=\"_blank\">50 nuances de grey</a> 🤭</p><strong>Une musique préférée&nbsp;?</strong><p>Des trucs qu'Ines ne connait pas la plupart du temps.</p><strong>Ta BD préférée&nbsp;?</strong><p>Je peux pas en parler, c'est chez la concurrence... <span style=\"font-size: 1.3em;\">🐗</span></p><strong>Ce que tu aimes chez Dupuis&nbsp;?</strong><p>Tout&nbsp;!</p><strong>Quelle est ta bonne résolution pour 2023&nbsp;?</strong><p>N'en prendre aucune, na&nbsp;!</p>".to_string(),
            sound: "caroline.mp3".to_string()
        },
        Card {
            picture: "Celine-mini.jpg".to_string(),
            sprite: "Celine.png".to_string(),
            name: "Céline".to_string(),
            description: "<p><i>\"Ça se tente&nbsp;!\"</i></p><strong>Tu fais quoi de tes journées&nbsp;?</strong><p>Je m'amuse, je ris et je lis&nbsp;!</p><strong>Une passion&nbsp;?</strong><p>Aucune&nbsp;! J'aime trop de choses pour être passionnée par une seule.</p><strong>À la maison il y a&nbsp;?</strong><p>1 chéri, 2 princesses et beaucoup trop de poupées et de peluches… 🐈</p><strong>Un pseudo&nbsp;?</strong><p>Ni pseudo, ni diminutif. On m'appelle Céline ou on m'appelle pas&nbsp;!&nbsp;:-D</p><strong>Un dicton/mantra&nbsp;?</strong><p>Dans le boulot&nbsp;: «&nbsp;Ça se tente&nbsp;!&nbsp;»<br/>Dans la vie&nbsp;: « C'est pas grave. »</p><strong>Un film préféré&nbsp;?</strong><p><a href=\"https://youtu.be/Nt2UXoNtadQ\" target=\"_blank\">Les Noces Funèbres</a></p><strong>Une musique préférée&nbsp;?</strong><p>En 4 lettres&nbsp;: <a href=\"https://youtu.be/piXD9RlBPo4\" target=\"_blank\">SAEZ</a></p><strong>Ta BD préférée&nbsp;?</strong><p>Mes coups de coeur de l'année sont&nbsp;:<br/>Webtoon&nbsp;: Distress<br/>Manga&nbsp;: KakushiGoto<br/>BD 1&nbsp;: Portugal<br/>BD 2&nbsp;: Toutes les princesses meurent après minuit<br/></p><strong>Ce que tu aimes chez Dupuis&nbsp;?</strong><p>M'amuser (grâce à mon job) , rire (grâce à mes collègues) et lire (grâce à nos BDs)&nbsp;!</p><strong>Quelle est ta bonne résolution pour 2023&nbsp;?</strong><p>M'amuser plus, rire plus et lire plus&nbsp;!</p>".to_string(),
            sound: "celine.mp3".to_string()
        },
        Card {
            picture: "Ines-mini.jpg".to_string(),
            sprite: "Ines.png".to_string(),
            name: "Inès".to_string(),
            description: "<p><i>\"Besoin d'une belle mise en situ&nbsp;?\"</i></p><strong>Tu fais quoi de tes journées&nbsp;?</strong><p>Je passe mes journées sur mon téléphone, quelle belle vie&nbsp;!</p><strong>Une passion&nbsp;?</strong><p>La nourriture, ça compte&nbsp;?</p><strong>À la maison il y a&nbsp;?</strong><p>Un lit, une douche, une cuisine... Et mon boyfriend, évidemment&nbsp;!</p><strong>Un pseudo&nbsp;?</strong><p>@Inesfiestadelagravasloca</p><strong>Un dicton/mantra&nbsp;?</strong><p>Ta deuxième vie commence quand tu comprends que tu n'en n'as qu'une</p><strong>Un film préféré&nbsp;?</strong><p><a href=\"https://youtu.be/DD0m9t4WHEQ\" target=\"_blank\">Le Grinch</a></p><strong>Ta musique préférée&nbsp;?</strong><p>Le générique de notre émission \"<a href=\"https://youtu.be/hej6GkGGuWQ&list=PLfv0zkBFPxXCDD-2SRoLFbxEv6gkl4mEu\" target=\"_blank\">Avec ou sans bulles</a>\"&nbsp;!</p><strong>Ta BD préférée&nbsp;?</strong><p><a href=\"https://www.dupuis.com/dad/bd/dad-tome-1-filles-a-papa/62195#lecteurbd\" target=\"_blank\">Dad</a></p><strong>Ce que tu aimes chez Dupuis&nbsp;?</strong><p>Mon équipe, la convivialité, la nouvelle machine à café, les soupes en hiver</p><strong>Quelle est ta bonne résolution pour 2023&nbsp;?</strong><p>Moins râler, plus profiter de la vie&nbsp;!</p>".to_string(),
            sound: "ines.mp3".to_string()
        },
        Card {
            picture: "Natacha-mini.jpg".to_string(),
            sprite: "Natacha.png".to_string(),
            name: "Natacha".to_string(),
            description: "<p><i>\"C'est très bien, faut juste changer la typo&nbsp;!\"</i></p><strong>Tu fais quoi de tes journées&nbsp;?</strong><p>Je donne vie à des dessins en écoutant de la musique&nbsp;!</p><strong>Une passion&nbsp;?</strong><p>Plusieurs même&nbsp;! Écriture, lecture, dessin, musique, cinéma, théâtre...</p><strong>À la maison il y a&nbsp;?</strong><p>Un planétarium de chambre, une estampe japonaise et un tiroir rempli de biscuits au chocolat</p><strong>Un pseudo&nbsp;?</strong><p>La danseuse folle</p><strong>Un dicton/mantra&nbsp;?</strong><p>\"Il faut toujours viser la lune, car même en cas d'échec, on atterrit dans les étoiles.\" Oscar Wilde</p><strong>Un film préféré&nbsp;?</strong><p><a href=\"https://youtu.be/8eZpsPixVuU\" target=\"_blank\">Rimbaud Verlaine (Total Eclipse)</a></p><strong>Ta musique préférée&nbsp;?</strong><p><a href=\"https://youtu.be/DeboAPrCjbo\" target=\"_blank\">Clann - Her & the Sea</a></p><strong>Ta BD préférée&nbsp;?</strong><p><a href=\"https://www.dupuis.com/les-campbell/bd/les-campbell-tome-1-inferno/54694#lecteurbd\" target=\"_blank\">Les Campbell</a></p><strong>Ce que tu aimes chez Dupuis&nbsp;?</strong><p>Mon équipe, recevoir le journal Spirou chaque semaine, la déco des bureaux, les restes de tarte au frigo</p><strong>Quelle est ta bonne résolution pour 2023&nbsp;?</strong><p>Apprendre à jouer du violon&nbsp;!</p>".to_string(),
            sound: "natacha.mp3".to_string()
        },
        Card {
            picture: "Sandrine-mini.jpg".to_string(),
            sprite: "Sandrine.png".to_string(),
            name: "Sandrine".to_string(),
            description: "<p><i>\"Shit In Shit Out\"</i></p><strong>Tu fais quoi de tes journées&nbsp;?</strong><p>Je passe mes journées devant mon ordi pas (seulement) pour regarder des séries Nexflix mais surtout pour animer nos belles séries afin que les gens achètent plus d’albums&nbsp;!!!</p><strong>Une passion&nbsp;?</strong><p>Ennuyer les gens avec mes questions débiles&nbsp;🙂<br/>«&nbsp;Qu’est-ce que tu faiiiiis&nbsp;?&nbsp;»</p><strong>À la maison il y a&nbsp;?</strong><p>Trois enfants dont 1 de 37 ans.</p><strong>Un pseudo&nbsp;?</strong><p>Sandrinette/ Choupette</p><strong>Un dicton/mantra&nbsp;?</strong><p>Shit In Shit Out</p><strong>Un film préféré&nbsp;?</strong><p>Un amour de Coccinelle avec Choupette</p><strong>Une musique préférée&nbsp;?</strong><p><a href=\"https://youtu.be/rn_YodiJO6k\" target=\"_blank\">Otherside - Red Hot Chili Peppers</a></p><strong>Ta BD préférée&nbsp;?</strong><p><a href=\"https://www.dupuis.com/seriebd/louca/5127\" target=\"_blank\">La série Louca</a></p><strong>Ce que tu aimes chez Dupuis&nbsp;?</strong><p>La nouvelle machine à café avec le cacao fort, les sandwichs,...<br/>Mais surtout, notre équipe, les collègues sans qui ça n’aurait pas la même saveur&nbsp;!</p><strong>Quelle est ta bonne résolution pour 2023&nbsp;?</strong><p>Résolu…quoi&nbsp;?</p>".to_string(),
            sound: "sandrine.mp3".to_string()
        },
        Card {
            picture: "Marc-mini.jpg".to_string(),
            sprite: "Marc.png".to_string(),
            name: "Marc<div>Graphiste, Motion Designer et Edito Webtoon</div>".to_string(),
            description: "<p><i>\"Travaille actuellement sous couverture&nbsp;!\"</i></p><strong>Tu fais quoi de tes journées&nbsp;?</strong><p>Je me vois comme une sorte de couteau suisse créatif.<br/>Toujours prêt pour la prochaine présentation farfelue.<br/>Je remplis aussi 500 gigas de données par an.<br/>Alain, je n’ai plus d’espace disque&nbsp;!</p><strong>Une passion&nbsp;?</strong><p><a href=\"https://bvr.yo.fr/compilations/\" target=\"_blank\">Faire des compilations musicales.</a></p><strong>À la maison il y a&nbsp;?</strong><p>Plein de filles&nbsp;!<br/>Morgan, Claire, Émilie, trois poules et deux chattes. Donc, c’est moi qui fais la lessive et le repassage.</p><strong>Un pseudo&nbsp;?</strong><p><a href=\"https://bvr.yo.fr/app2022/data/Marc/Marc05.jpg\" target=\"_blank\">Marc Garitas</a>, c’est mon pseudo de Réf (un arbitre quoi) au Roller Derby. </p><strong>Un dicton/mantra&nbsp;?</strong><p>Le digital, c’est pour les fleuristes&nbsp;!</p><strong>Un film préféré&nbsp;?</strong><p><a href=\"https://youtu.be/07-QBnEkgXU\" target=\"_blank\">Eternal sunshine of the spotless mind</a></p><strong>Une musique préférée&nbsp;?</strong><p><a href=\"https://youtu.be/w6YhNcEbOHE\" target=\"_blank\">Roy Vedas- Fragment of Life</a></p><strong>Ta BD préférée&nbsp;?</strong><p><a href=\"https://bdi.dlpdomain.com/player/1pc61bTiTNFyataHFUiufeUYzN1U8dX0.html\" target=\"_blank\">'Seconds' de Bryan Lee O’Malley</a></p><strong>Ce que tu aimes chez Dupuis&nbsp;?</strong><p>Le café&nbsp;! Mais aussi le contact créatif avec les auteurs. Le partage avec les collègues, surtout si ils apportent de la tarte.</p><strong>Quelle est ta bonne résolution pour 2023&nbsp;?</strong><p><a href=\"https://wikiagile.fr/images/thumb/1/1c/The-four-toltec-agreements_fr.png/597px-The-four-toltec-agreements_fr.png\" target=\"_blank\">Suivre les 4 accords Toltèques&nbsp;!</a></p>".to_string(),
            sound: "marioJump.mp3".to_string()
        },
        Card {
            picture: "Alexis-mini.jpg".to_string(),
            sprite: "Alexis.png".to_string(),
            name: "Alexis<div>Dev_</div>".to_string(),
            description: "<p><i>\"In Rust We Trust 🦀\"</i></p><strong>Tu fais quoi de tes journées&nbsp;?</strong><p>Tac tac tac...</p><strong>Une passion&nbsp;?</strong><p>Le DIY</p><strong>À la maison il y a&nbsp;?</strong><p>Moi</p><strong>Un pseudo&nbsp;?</strong><p>└[∵┌]└[ ∵ ]┘[┐∵]┘</p><strong>Un dicton/mantra&nbsp;?</strong><p>Y a moyen de gagner encore un octet</p><strong>Un film préféré&nbsp;?</strong><p><a href=\"https://youtu.be/JJkPLYmUyzg\" target=\"_blank\">Enter the Void</a></p><strong>Une musique préférée&nbsp;?</strong><p><a href=\"https://youtu.be/ZmWYCIZhBgk\" target=\"_blank\">All is Violent, All is Bright - God Is An Astronaut</a></p><strong>Ta BD préférée&nbsp;?</strong><p><a href=\"https://www.petitapetit.fr/produit/rip-t1/\" target=\"_blank\">RIP</a></p><strong>Ce que tu aimes chez Dupuis&nbsp;?</strong><p>C'est accueillant</p><strong>Quelle est ta bonne résolution pour 2023&nbsp;?</strong><p>REDACTED</p>".to_string(),
            sound: "alexis.mp3".to_string()
        }
    ];
}
