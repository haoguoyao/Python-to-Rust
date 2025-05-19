import os
import json
import pprint
from annotated_toml_module import load, loads, TomlDecoder, TomlDecodeError

def test_load_file():
    """Test loading TOML from a file"""
    try:
        with open('example-v0.4.0.toml', 'r', encoding='utf-8') as f:
            data = load(f)
        print(data)
        return data
    except TomlDecodeError as e:
        print(f"Error loading TOML file: {e}")
        return None



def main():

    print("Testing TOML decoder with example-v0.4.0.toml")
    
    # Test different loading methods
    data1 = test_load_file()
  
if __name__ == "__main__":
    main()