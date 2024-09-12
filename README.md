# Stingray Material Texture Hash Check

> [!Note]
> This is a companion tool for limn and credits go to [manshanko](https://github.com/manshanko) who wrote all the core code

This tool allows you to check Warhammer 40,000: Darktide material files for the textures they contain. It helps with grabbing specific textures instead of having to look through a random list manually.

Not a very imaginative name but this repository is just a glorified way to distribute the exe safely 😅 I plan to make a mod that makes it easier to grab material IDs as well.

## Usage

> [!Important]
> A minimum of **74GB** free storage space is required to begin. Output files can be deleted but if you wish to keep a copy of all textures as PNG, they will take up 63GB.

1. Grab [limn](https://github.com/manshanko/limn)
2. _Without_ a `dictionary.txt` file in the same directory as `limn.exe`, run the following commands:
  - This will dump ~73GB of DDS textures to the `out` subdirectory

    ```bash
    .\limn texture
    ```

  - This will dump ~350MB of material files to the `out` subdirectory

		```bash
		.\limn material
		```


  - This will dump a ~450KB `hashes.bin` file to the current directory

		```bash
		.\limn --dump-hashes texture
		```
	
3. Get the material ID/s you want from the game, then fill in the rest of the owl (will make a mod for this later)
4. Search the `out` subdirectory for `.material` files matching those ID/s and copy them back into the same folder as `hashes.bin`
5. Copy [stingray-material-texture-hash-check.exe](https://github.com/ronvoluted/stingray-material-texture-hash-check/releases) to the same folder as `hashes.bin`
6. Run `stingray-material-texture-hash-check.exe` and it will list the textures these materials contain. Example output:
```
Found hashes.bin

8584945545715c1a.material:
  95c91701d347bc75
  2acaf70f8376624a
  cae37816235a47f2
9b82dbf3cb623bd9.material:
  fddbb2440b8513cf
  c80654f1f1076176
  3ba1a96eb7847797
```

## Converting from DDS

You can search the `out` folder for DDS files matching those texture IDs and convert them to another image format individually, then delete all the `.material`, `.dds` and `.bin` files.

If you'd like to keep a copy of all textures however, grab and setup [dds-to-img](https://github.com/ronvoluted/dds-to-img) which will convert all DDS files to PNG or WEBM. Besides making textures readily viewable, it has the added benefit of reducing total space usage to 86% (DDS -> PNG). You can delete all `.dds` afterwards.
