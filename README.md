# winrar-rs

### Description: library to handle compressed files for multi extensions

### Interfaces

* ArchiveReader: enumerate archived files, get infos and read them.
  * _info method returns a ArchiveInfo type about the compressed file that's include _length_, _comment_, _has password_ keys. 
  * _enumeratefiles method return a array with ArchiveFileInfo with _basedir, filename, index_ keys.
  * _readByindex_ method receive a index of file and returns buffered content.
* ArchiveWrite: __not implemented yet_


