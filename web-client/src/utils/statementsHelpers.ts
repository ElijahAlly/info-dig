import { LinkType } from '../interfaces/statements';

export const createReadableSlug = (content: string): string => {
    // Replace special characters with "-"
    let formattedContent = content.replace(/[^\w]+/g, '-');

    // Convert to lowercase
    formattedContent = formattedContent.toLowerCase();

    // Extended list of words to avoid ending with
    const ambiguousEndings = ["and", "or", "but", "unless", "though", "although", "so", "yet", "for", "nor", "as"];

    // Split by "-" and limit the number of words to make the slug of reasonable length
    let words = formattedContent.split('-').filter(word => word && !ambiguousEndings.includes(word));
    if (words.length > 5) {
        words = words.slice(0, 5);
    }

    // Remove ambiguous ending if necessary
    while (ambiguousEndings.includes(words[words.length - 1])) {
        words.pop();
    }

    // Rejoin the words to form the slug
    let slug = words.join('-');

    // Ensure the slug doesn't end with a dash
    slug = slug.replace(/-+$/, '');

    return slug;
};

const isValidUrl = (url: string) => { // TODO: Actually vist url to see if it's active and accessible
    // const urlRegex = /^(https?|ftp):\/\/[^\s/$.?#].[^\s]*$/i; // * https only
    const urlRegex = /^(https?:\/\/)[^\s/$.?#].[^\s]*$/i         // * Both https & http allowed
    return urlRegex.test(url);
}

export const containsValidLinks = (links: LinkType[]) => {
    if (!links.length || !links[0].url || !links[0].name) return [];
    const newLinks: LinkType[] = [];
    for (let i = 0; i < links.length; i++) {
        const curLink = links[i];
        if (isValidUrl(curLink.url) && curLink.name.length > 0) {
            newLinks.push(curLink);
        }
    }
    return newLinks;
}

// export const createReadableSlug = (content: string): string => {
//     // Define a regular expression for allowed special characters
//     const allowedSpecialCharsRegex = /[ .,:;(){}\[\]\\\|/"?'<>+=_*&^%$#@!`~]+/g;

//     // Replace allowed special characters with "-"
//     let formattedContent = content.replace(allowedSpecialCharsRegex, "-");

//     // Replace multiple "-" with a single "-"
//     formattedContent = formattedContent.replace(/-+/g, "-");

//     // Truncate to a maximum of 41 characters plus an extra "-" if needed
//     if (formattedContent.length > 41) {
//         formattedContent = formattedContent.substring(0, 41) + "-";
//     }

//     return formattedContent;
// }

export const getCorrectColor = (rating: string) => {
    switch (rating) {
        case 'Proven_Truth':
            return ({
                backgroundColor: 'rgb(55, 196, 239)',
                color: '#063948'
            })
        case 'In_Question':
            return ({
                backgroundColor: 'rgb(239, 190, 55)',
                color: '#063948'
            })
        case 'Not_True':
            return ({
                backgroundColor: 'rgb(239, 55, 104)',
                color: 'aliceblue'
            })
        default:
            return ({
                backgroundColor: 'gray',
                color: '#063948'
            })
    }
};

export const formatRating = (rating: string) => {
    if (!rating) return '';
    return rating.replace('_', ' ');
}