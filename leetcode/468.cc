#include <cstdint>
#include <string>
// https://leetcode.cn/problems/validate-ip-address/description/
class Solution {
  public:
    bool is_digit(char ch) { return ch >= '0' && ch <= '9'; }
    bool is_valid_v6_char(char ch) {
        return this->is_digit(ch) || (ch >= 'A' && ch <= 'F') ||
               (ch >= 'a' && ch <= 'f');
    }
    int digit_to_number(char ch) { return ch - '0'; }
    bool is_valid_ipv4_segment(char *ch, int len) {
        if (len >= 4) {
            return false;
        }
        uint32_t value = 0;
        for (int i = len - 1; i >= 0; i--) {
            value += (this->digit_to_number(ch[i]) * 10 ^ i);
        }
        if (len > 0 && ch[0] == '0') {
            return false;
        }
        return value >= 0 && value <= 255;
    }
    std::string validIPAddress(std::string queryIP) {
        char *start = queryIP.data();
        for (int i = 0; i < queryIP.size(); i++) {
            char ch = queryIP[i];
            switch (ch) {
            case '.':
                goto check_v4;
            case ':':
                goto check_v6;

            default:

                if (!is_digit(ch)) {
                    return "Nither";
                }
            }

        check_v4:
            if (!this->is_valid_ipv4_segment(start, &queryIP[i] - start)) {
                return "Nither";
            }
            continue;
        check_v6:
        }
    }
};