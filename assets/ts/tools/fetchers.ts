/**
 * ## Get the html template
 * 
 * @param file - string
 * 
 * @returns `Promise<string>`
 */
export async function getHtml(file: string): Promise<string> {
    const RESPONSE = await fetch(file);
    return await RESPONSE.text();
}

/**
 * ## Get the data from the needed file
 * 
 * @param fileName - string
 * 
 * @returns `Promise<T>`
 */
export async function getKData<T>(fileName: string): Promise<T> {
    const RESPONSE = await fetch(`./data/${fileName}.k`);
    const DATA = await RESPONSE.text();

    // Split our data file
    const DATASPLIT = DATA.split(/::+|\r\n/mig);
    
    // Assign the keys to their values
    let kData: {[key: string]: string} = {};
    for (let i = 0; i < DATASPLIT.length; i++) {
        if (i % 2)
            kData[DATASPLIT[i - 1]] = DATASPLIT[i];
    }

    return kData as T;
}
