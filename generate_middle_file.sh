monkeytype run ./test.py 

mkdir middle_files
monkeytype list-modules
monkeytype -v stub toml.decoder > middle_files/decoder.pyi 
monkeytype apply toml.decoder > middle_files/decoder_withtype.py
