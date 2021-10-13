# ctc-decoder

```
$ npm install --save ctc-decoder
```

## Usage

```js
>>> import { viterbi_search } from 'ctc-decoder'
>>>
>>> alphabet = "NACGT"
>>> posteriors = np.random.rand(100, len(alphabet)).astype(np.float32)
>>>
>>> seq, path = viterbi_search(posteriors, alphabet)
>>> seq
'ACACTCGCAGCGCGATACGACTGATCGAGATATACTCAGTGTACACAGT'
```

## Credits

powerby [fast-ctc-decode](https://github.com/nanoporetech/fast-ctc-decode)
