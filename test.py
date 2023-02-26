#Display a trie alphabetically
def displayTrie(trie, prefix=""):
    if trie.isWord:
        print prefix
    for letter in trie.children:
        displayTrie(trie.children[letter], prefix+letter)