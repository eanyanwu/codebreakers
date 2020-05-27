# Codebreakers

Implementation the various techniques mentioned in David Khan's book "The Codebreakers"


# What's in here

- [x] The Vigenere cipher
- [x] Letter frequency analysis


# Examples:

- Vigenere cipher

From what I understand, punctuation and whitespace were disgarded to thwart letter frequency analysis, so I tried to reproduce that here.

```
# Write some plain text
echo "In an obscure corner of the Balkans, someone helpfully slew an archduke, and the nations leaped recklessly into the bloody cockpit of war" > plain_text.txt

# Encipher it and save the result to a file
./codebreakers vigenere encipher --key "THIS IS MODERN WAR" < plain_text.txt > cipher_text.txt

# Output
cat cipher_text.txt
> BUIFW TEQXV VPKRE XYWXB ZQPDP
> BNJSJ HTMGV WTSOT WHHLP LSMOI
> FMFFL UHGER GKBZM FMHLS EFHER
> ILLJM UWZHW JYUIE MVBZM TXCRH
> PPKCB IPBGN OMF

# Decipher
./codebreakers vigenere decipher --key "THIS IS MODERN WAR" < cipher_text.txt
> INANO BSCUR ECORN EROFT HEBAL
> KANSS OMEON EHELP FULLY SLEWA
> NARCH DUKEA NDTHE NATIO NSLEA
> PEDRE CKLES SLYIN TOTHE BLOOD
> YCOCK PITOF WAR
```

- Letter frequency analysis:

It's interesting to see how the histogram differs when you give it plain text vs cipher text.

```
./codebreakers letter-frequency < plain_text.txt

> A |||||||||
> B |||
> C ||||||
> D ||||
> E ||||||||||||||
> F |||
> G
> H |||||
> I ||||
> J
> K ||||
> L |||||||||
> M |
> N ||||||||||
> O |||||||||||
> P |||
> Q
> R ||||||
> S |||||||
> T ||||||
> U |||
> V
> W ||
> X
> Y |||
> Z

./codebreakers letter-frequency < cipher_text.txt
> A
> B |||||||
> C ||
> D |
> E ||||||
> F |||||||
> G ||||
> H ||||||||
> I |||||
> J ||||
> K |||
> L ||||||
> M |||||||||
> N ||
> O |||
> P |||||||
> Q ||
> R ||||
> S ||||
> T |||||
> U ||||
> V ||||
> W ||||||
> X ||||
> Y ||
> Z ||||
```