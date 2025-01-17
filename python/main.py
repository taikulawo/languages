from req import *;
import file

def main():
    print("Hello from example!")
    r = send()
    print("status_code:{0}, content-type:{1}".format(r.status_code,r.headers['content-type']))
    content = file.read_current_file()
    print("current file first line:\n{}".format(content))


if __name__ == "__main__":
    main()
