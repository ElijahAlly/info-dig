export const API_BASE_URL = 'http://localhost:8081'; // in prod use 'https://info-dig.com'

export const formatContentToSlug = (content: string): string => {
    // Define a regular expression for allowed special characters
    const allowedSpecialCharsRegex = /[ .,:;(){}\[\]\\\|/"?'<>+=_*&^%$#@!`~]+/g;

    // Replace allowed special characters with "-"
    let formattedContent = content.replace(allowedSpecialCharsRegex, "-");

    // Replace multiple "-" with a single "-"
    formattedContent = formattedContent.replace(/-+/g, "-");

    // Truncate to a maximum of 41 characters plus an extra "-" if needed
    if (formattedContent.length > 41) {
        formattedContent = formattedContent.substring(0, 41) + "-";
    }

    return formattedContent;
}
