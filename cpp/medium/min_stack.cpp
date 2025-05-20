#include <unordered_map>
#include <vector>

class MinStack {
    public:
        MinStack(): _counters(), _stack() {
            
        }
        
        void push(int val) {
            if (_stack.empty()) {
                // It is guaranteed that pop/getMin/top wouldn't be called on empty stack, so the value in this case doesn't matter
                _min = val;
                _counters[_min] = std::make_pair(1, INT32_MIN);
            } else {
                if (_min == val) {
                    _counters[_min].first++;
                } else if (_min > val) {
                    _counters[val] = std::make_pair(1, _min);
                    _min = val;
                } else if (_counters.find(val) != _counters.end())
                    _counters[val].first++;
            }
            _stack.push_back(val);
        }
        
        void pop() {
            int top = this->top();
            if (top == _min && _counters.find(top) != _counters.end()){
                if (_counters[top].first == 1) {
                    _min = _counters[top].second;
                    _counters.erase(top);
                } else {
                    _counters[top].first--;
                }
            }
            _stack.pop_back();
        }
        
        int top() {
            return _stack.back();
        }
        
        int getMin() {
            return _min;
        }
    private:
        // element | count | prev_min   
        std::unordered_map<int, std::pair<int, int>> _counters;
        std::vector <int> _stack;
        int _min;
    };
    
    /**
     * Your MinStack object will be instantiated and called as such:
     * MinStack* obj = new MinStack();
     * obj->push(val);
     * obj->pop();
     * int param_3 = obj->top();
     * int param_4 = obj->getMin();
     */

#include <cassert>

int main() {
    MinStack minStack;
    minStack.push(-2);
    minStack.push(0);
    minStack.push(-3);
    assert(minStack.getMin() == -3);
    minStack.pop();
    assert(minStack.top() == 0);
    assert(minStack.getMin() == -2);

    MinStack minStack2;
    minStack2.push(512);
    assert(minStack2.getMin() == 512);
    minStack2.push(-1024);
    minStack2.push(-1024);
    minStack2.push(512);
    assert(minStack2.getMin() == -1024);
    minStack2.pop();
    assert(minStack2.getMin() == -1024);
    minStack2.pop();
    assert(minStack2.getMin() == -1024);
    minStack2.pop();
    assert(minStack2.getMin() == 512);
    return 0;
}