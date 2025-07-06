struct File {
  int fd;
};

struct File* open_bugged() {
  struct File f = { open("README.md", O_RDONLY) };
  return &f;
}
