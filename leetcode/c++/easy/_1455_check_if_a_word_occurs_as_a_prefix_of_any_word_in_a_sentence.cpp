// Leetcode Problem 1455

#include <gtest/gtest.h>
#include <string>
#include <sstream>
#include <vector>
#include <regex>

using namespace std;

class Solution {
public:
    int isPrefixOfWord(string sentence, string searchWord) {
        int i = -1, wordIndex = 0;

        do {
            ++i, ++wordIndex;
            if (sentence.substr(i, searchWord.size()) == searchWord) {
                return wordIndex;
            }
        } while ((i = sentence.find(' ', i)) != string::npos);

        return -1;
    }

    /**
     * Solution i came up with but is very slow
     */
    int isPrefixOfWord_regexMatching(string sentence, string searchWord) {
        if (sentence.length() < searchWord.length()) {
            return -1;
        }

        int position = 1;

        string word;
        istringstream stream(sentence);

        string escapedWord = regex_replace(searchWord, regex("([\\^\\.\\$\\|\\(\\)\\[\\]\\\\\\+\\*\\?\\{\\}])"), "\\$1");
        string pattern = "^" + escapedWord;

        regex regexPattern(pattern);

        while (stream >> word) {
            if (regex_search(word, regexPattern)) {
                return position;
            }

            position++;
        }

        return -1;
    }
};


class TestClass : public ::testing::Test {
protected:
    Solution solution;
};

TEST_F(TestClass, SimplePossibleCase) {
    string sentence = "i love eating burgers";
    string seachWord = "burg";

    EXPECT_EQ(solution.isPrefixOfWord(sentence, seachWord), 4);
}

TEST_F(TestClass, SimplePossibleCase2) {
    string sentence = "this problem is an easy problem";
    string searchWord = "pro";

    EXPECT_EQ(solution.isPrefixOfWord(sentence, searchWord), 2);
}

TEST_F(TestClass, ImpossibleCase) {
    string sentence = "i am tired";
    string searchWord = "you";

    EXPECT_EQ(solution.isPrefixOfWord(sentence, searchWord), -1);
}

TEST_F(TestClass, SearchWordTooLong) {
    string sentence = " ";
    string searchWord = "you";

    EXPECT_EQ(solution.isPrefixOfWord(sentence, searchWord), -1);
}

TEST_F(TestClass, EmptySentence) {
    string sentence = " ";
    string seachWord = "a";

    EXPECT_EQ(solution.isPrefixOfWord(sentence, seachWord), -1);
}