export const keyHash = (key: string) => {
    // modified from https://stackoverflow.com/a/52171480

    let h1 = 0x243f6a88, h2 = 0x85a308d3;
    for (let i = 0, ch; i < key.length; i++) {
        ch = key.charCodeAt(i);
        h1 = Math.imul(h1 ^ ch, 2654435761);
        h2 = Math.imul(h2 ^ ch, 1597334677);
    }
    h1 = Math.imul(h1 ^ (h1 >>> 16), 2246822507);
    h1 ^= Math.imul(h2 ^ (h2 >>> 13), 3266489909);
    h2 = Math.imul(h2 ^ (h2 >>> 16), 2246822507);
    h2 ^= Math.imul(h1 ^ (h1 >>> 13), 3266489909);

    return 4294967296 * (2097151 & h2) + (h1 >>> 0);
};

export const keyColor = (key: string): string => {
    let hash = keyHash(key);

    return `oklch(80% 90% ${(hash / 3600) % 360.0})`;
}
