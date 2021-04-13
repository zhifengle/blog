#include <fstream>
#include <iostream>
#include <string>

using namespace std;
string readWholeFile(const char* name);

int main(int argc, char const* argv[]) {
    auto query = argv[1];
    auto filename = argv[2];
    cout << readWholeFile(filename) << endl;
    return 0;
}

// https://stackoverflow.com/questions/2912520/read-file-contents-into-a-string-in-c
string readWholeFile(const char* name) {
    ifstream infile(name, ios::in);
    if (!infile) {
        // TODO deal error
        // cerr << "open error: " << name << endl;
        // exit(1);
    }
    /*
    ifstream infile;
    infile.open("afile.dat");
    cout << "Reading from the file" << endl;
    infile >> data;
    // 在屏幕上写入数据
    // 只有读取一部分。遇到空格就结束
    cout << data << endl;
    infile.close();
    */
    /* 分开的写法 */
    // string contents;
    // contents.assign(istreambuf_iterator<char>(infile),
    //                 istreambuf_iterator<char>());
    // 第一个参数需要用括号包起来
    // STL
    string contents((istreambuf_iterator<char>(infile)),
                    (istreambuf_iterator<char>()));
    // RAII object ??
    // 似乎不用手动调用
    // automatically by the ifstream destructor. 如果占用太多资源，可以手动调用
    // infile.close();
    return contents;
}
