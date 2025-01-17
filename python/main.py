import requests
def send():
    r = requests.get('https://httpbin.org/basic-auth/user/pass', auth=('user', 'pass'))
    return r
def main():
    print("Hello from example!")
    r = send()
    print("status_code:{0}, content-type:{1}".format(r.status_code,r.headers['content-type']))


if __name__ == "__main__":
    main()
