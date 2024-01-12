#include <cmath>
#include <cstdint>
#include <string>
#include <iostream>
// https://leetcode.cn/problems/validate-ip-address/description/
class Solution {
  public:
    bool is_digit(char ch) { return ch >= '0' && ch <= '9'; }
    bool is_valid_v6_char(char ch) {
        return this->is_digit(ch) || (ch >= 'A' && ch <= 'F') ||
               (ch >= 'a' && ch <= 'f');
    }
    bool is_valid_v6_segment(char *ch, int len) {
        if (len > 4) {
            return false;
        }
        for (int i = 0; i < len; i++) {
            if (!this->is_valid_v6_char(ch[i])) {
                return false;
            }
        }
        return true;
    }
    int digit_to_number(char ch) { return ch - '0'; }
    bool is_valid_ipv4_segment(char *ch, int len) {
        if (len >= 4) {
            return false;
        }
        uint32_t value = 0;
        for (int i = len - 1; i >= 0; i--) {
            if (!this->is_digit(ch[i])) {
                return false;
            }
            value += (this->digit_to_number(ch[i]) * std::pow(10, i));
        }
        if (len > 0 && ch[0] == '0') {
            return false;
        }
        return value >= 0 && value <= 255;
    }
    std::string validIPAddress(std::string queryIP) {
        char *start = queryIP.data();
        char *end;
        int len = 0;
        bool not_ipv4 = false, not_ipv6 = false;
        int v4_check_count = 4;
        int v6_check_count = 8;
        for (int i = 0;
             i < queryIP.size() && start < (queryIP.data() + queryIP.size());
             i++) {
            char ch = queryIP[i];
            switch (ch) {
            case '.':
                if (not_ipv4) {
                    goto errout;
                }
                v4_check_count--;
                not_ipv6 = true;
                goto check_v4;
            case ':':
                if (not_ipv6) {
                    goto errout;
                }
                v6_check_count--;
                not_ipv4 = true;
                goto check_v6;

            default:
                // check final segment
                if (i == queryIP.size() - 1) {
                    // last
                    if (not_ipv4 && not_ipv6) {
                        goto errout;
                    }
                    if (not_ipv4) {
                        goto check_v6;
                    } else if (not_ipv6) {
                        goto check_v4;
                    }
                }
                continue;
            }

        check_v4:
            if (!this->is_valid_ipv4_segment(start, &queryIP[i] - start)) {
                goto errout;
            }
            if (v4_check_count < 0) {
                goto errout;
            } else if (v4_check_count == 0) {
                // last time
                return "IPv4";
            }
            start = &queryIP[i] + 1;
            continue;
        check_v6:
            if (!this->is_valid_v6_segment(start, &queryIP[i] - start)) {
                goto errout;
            }
            if (v6_check_count < 0) {
                goto errout;
            } else if (v6_check_count == 0) {
                return "IPv6";
            }
            start = &queryIP[i] + 1;
        }
    errout:
        return "Neither";
    }
};

int main(int argc, char *argv[]) {
    std::string str("172.16.254.1");
    Solution s;
    std::string res = s.validIPAddress(str);
    std::cout << res;
}