from req import *;
import file
from bs4 import BeautifulSoup

def main():
    soup = BeautifulSoup("<p>Some<b>bad<i>HTML")
    print("Hello from example!")
    r = send()
    print("status_code:{0}, content-type:{1}".format(r.status_code,r.headers['content-type']))
    content = file.read_current_file()
    print("current file first line: {}".format(content))
    print(soup.prettify())
    soup.find(string="bad")

if __name__ == "__main__":
    main()
