export async function getPublicKey() {
    try {
        const signedEventData = await window.nostr.getPublicKey();
        return signedEventData;
    } catch (error) {
        console.error("Error getting public key:", error);
        throw error;
    }
}

export async function signEvent(event) {
    try {
        // Implement your logic here to sign the event
        const eventObj = JSON.parse(event);
        const signedEventData = await window.nostr.signEvent(eventObj);
        return JSON.stringify(signedEventData);
    } catch (error) {
        console.error("Error signing event:", error);
        throw error;
    }
}

