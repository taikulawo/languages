import requests
def send():
    r = requests.get('https://httpbin.org/basic-auth/user/pass', auth=('user', 'pass'))
    return r