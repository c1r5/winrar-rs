# winrar-rs

### Description: library to handle compressed files for multi extensions

### Interfaces

* ```ArchiveReader```: enumerate archived files, get infos and read them.

  * ```info()``` method returns a ArchiveInfo type about the compressed file that's include ```_length_, _comment_, _has password_``` keys. 
  * ```enumeratefiles()``` method return a array with ArchiveFileInfo with ```basedir, filename, index``` keys.
  * ```readByindex()``` method receive a index of file and returns buffered content.
* ```ArchiveWrite```: _not implemented yet_

### To-Do list
- [x] Get info about ```zip``` file.
- [x] List files inside of archive.
- [x] Read file into archive

