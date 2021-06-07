# word_sensitive
A library for sensitive string matching, the implementation language is rust, and the algorithm used is ac

# How to use?

* Add keywords to trie
* Build trie
* Query(match keyword from text)
## Manually add keywords

```rust
  use word_sensitive::trie;
  let mut tree = trie::Trie::default();
  tree.add_key_word("aaa".as_bytes().to_vec());
  tree.add_key_word("aab".as_bytes().to_vec());
  tree.add_key_word("aac".as_bytes().to_vec());
  tree.add_key_word("aba".as_bytes().to_vec());
  tree.add_key_word("abb".as_bytes().to_vec());
  tree.add_key_word("abc".as_bytes().to_vec());
  tree.add_key_word("aca".as_bytes().to_vec());
  tree.add_key_word("acb".as_bytes().to_vec());
  tree.add_key_word("acc".as_bytes().to_vec());
  tree.add_key_word("baa".as_bytes().to_vec());
  tree.add_key_word("bab".as_bytes().to_vec());
  tree.add_key_word("bac".as_bytes().to_vec());
  tree.add_key_word("bba".as_bytes().to_vec());
  tree.add_key_word("bbb".as_bytes().to_vec());
  tree.add_key_word("bbc".as_bytes().to_vec());
  tree.add_key_word("bca".as_bytes().to_vec());
  tree.add_key_word("bcb".as_bytes().to_vec());
  tree.add_key_word("bcc".as_bytes().to_vec());
  tree.add_key_word("caa".as_bytes().to_vec());
  tree.add_key_word("cab".as_bytes().to_vec());
  tree.add_key_word("cac".as_bytes().to_vec());
  tree.add_key_word("cba".as_bytes().to_vec());
  tree.add_key_word("cbb".as_bytes().to_vec());
  tree.add_key_word("cbc".as_bytes().to_vec());
  tree.add_key_word("cca".as_bytes().to_vec());
  tree.add_key_word("ccb".as_bytes().to_vec());
  tree.add_key_word("ccc".as_bytes().to_vec());
  tree.build();
  let matches = tree.query("abcabcbcca".as_bytes().as_ref());
  assert_eq!(matches[0], "abc".as_bytes().as_ref());
  assert_eq!(matches[1], "bca".as_bytes().as_ref());
  assert_eq!(matches[2], "cab".as_bytes().as_ref());
  assert_eq!(matches[3], "abc".as_bytes().as_ref());
  assert_eq!(matches[4], "bcb".as_bytes().as_ref());
  assert_eq!(matches[5], "cbc".as_bytes().as_ref());
  assert_eq!(matches[6], "bcc".as_bytes().as_ref());
  assert_eq!(matches[7], "cca".as_bytes().as_ref());

```

## Add keywords from file

```rust
use word_sensitive::trie;
let mut tree = trie::Trie::default();
tree.add_key_word_from_file("key_words/keywords.txt").unwrap();
tree.build();
let matches = tree.query("回民吃猪肉".as_bytes().as_ref());
assert_eq!(matches[0], "回民".as_bytes().as_ref());
assert_eq!(matches[1], "回民吃猪肉".as_bytes().as_ref());
```