// https://leetcode.com/problems/lfu-cache/

// Runtime
// 90 ms
// Beats 89.49%

// Memory
// 189.71 MB
// Beats 56.09%


#include <array>
#include <cassert>
#include <iostream>
#include <list>
#include <tuple>
#include <unordered_map>
#include <vector>

class LFUCache {
public:
    LFUCache(int capacity): _capacity(capacity), _count(0) {
        _cache.reserve(_capacity);
    }
    
    int get(int key) {
        int result = -1;

        auto it = _cache.find(key);

        if (it != _cache.end()){
            result = std::get<0>(it->second);
            auto freq_node = std::get<1>(it->second);
            int counter = freq_node->first + 1;
            auto next = std::next(freq_node);
            if (next != _freq_list.end() && next->first == counter) {
                next->second.push_front(key);
                std::get<1>(it->second) = next;
            } else {
                _freq_list.insert(next, std::make_pair(counter, std::list<int>(1, key)));
                std::get<1>(it->second) = std::prev(next);
            }
            freq_node->second.erase(std::get<2>(it->second));
            std::get<2>(it->second) = std::get<1>(it->second)->second.begin();
            if (freq_node->second.empty())
                _freq_list.erase(freq_node);
        }
        
        return result;
    }
    
    void put(int key, int value) {
        auto it = _cache.find(key);
        if (it != _cache.end()) {
            std::get<0>(it->second) = value;
            auto freq_node = std::get<1>(it->second);
            int counter = freq_node->first + 1;
            auto next = std::next(freq_node);
            if (next != _freq_list.end() && next->first == counter) {
                next->second.push_front(key);
                std::get<1>(it->second) = next;
            } else  {
                _freq_list.insert(next, std::make_pair(counter, std::list<int>(1, key)));
                std::get<1>(it->second) = std::prev(next);
            }
            freq_node->second.erase(std::get<2>(it->second));
            std::get<2>(it->second) = std::get<1>(it->second)->second.begin();
            if (freq_node->second.empty())
                _freq_list.erase(freq_node);
        } else {
            if (_count < _capacity){
                _count++;
                auto begin_lfu = _freq_list.begin();
                if (begin_lfu->first == 1) {
                    begin_lfu->second.push_front(key);
                    _cache[key] = std::make_tuple(value, begin_lfu, begin_lfu->second.begin());
                } else {
                    _freq_list.insert(begin_lfu, std::make_pair(1, std::list<int>(1, key)));
                    _cache[key] = std::make_tuple(value, std::prev(begin_lfu), std::prev(begin_lfu)->second.begin());
                }
            } else {
                auto begin_lfu = _freq_list.begin();
                if (begin_lfu->first == 1) {
                    auto evicted = begin_lfu->second.back();
                    begin_lfu->second.pop_back();
                    _cache.erase(evicted);

                    begin_lfu->second.push_front(key);
                    _cache[key] = std::make_tuple(value, begin_lfu, begin_lfu->second.begin());
                } else {
                    auto evicted = begin_lfu->second.back();
                    begin_lfu->second.pop_back();
                    _freq_list.insert(begin_lfu, std::make_pair(1, std::list<int>(1, key)));
                    
                    _cache[key] = std::make_tuple(value, std::prev(begin_lfu), std::prev(begin_lfu)->second.begin());
                    _cache.erase(evicted); 
                }
            }
        }
    }

private:
    int16_t _capacity;
    int16_t _count;

    // key: value|_freq_list::iterator|freq_node::iterator
    std::unordered_map<int, std::tuple<int, std::list<std::pair<int, std::list<int>>>::iterator, std::list<int>::iterator>> _cache;

    // counter: list_of_items
    std::list<std::pair<int, std::list<int>>> _freq_list;
};
/**
 * Your LFUCache object will be instantiated and called as such:
 * LFUCache* obj = new LFUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */

 int main() {
    LFUCache* obj = new LFUCache(2);
    obj->put(1,1);
    obj->put(2,2);
    assert(obj->get(1) == 1);
    assert(obj->get(2) == 2);
    assert(obj->get(2) == 2);
    obj->put(3,3);
    assert(obj->get(2) == 2);
    assert(obj->get(1) == -1);
    assert(obj->get(3) == 3);

    obj->put(1,1);
    assert(obj->get(2) == 2);
    assert(obj->get(1) == 1);
    assert(obj->get(3) == -1);

    delete obj;

    obj = new LFUCache(3);
    obj->put(2,2);
    obj->put(1,1);
    assert(obj->get(2) == 2);
    assert(obj->get(1) == 1);
    assert(obj->get(2) == 2);
    obj->put(3,3);
    obj->put(4,4);
    assert(obj->get(3) == -1);
    assert(obj->get(2) == 2);
    assert(obj->get(1) == 1);
    assert(obj->get(4) == 4);

    for (int i = 0; i< 500000; i++){
        obj->get(4);
        obj->put(3,3);
        obj->put(2,2);
        obj->get(1);
        obj->get(1);
        obj->put(5,5);
    }

    return 0;
 }