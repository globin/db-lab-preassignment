/* Task 0: Mini-Hekaton

This file contains an incomplete implementation of a storage backend
for a relation with three attributes (a, b, and c). Each row is
dynamically allocated on the heap. For attribute a the relation has a
hash index which stores pointers to the rows. Rows with the same hash
value are linked using the next field and can only be accessed through
the hash table (to scan all rows one has to iterate through the hash
table). This storage format is used in Hekaton, the main-memory engine
of Microsoft SQL Server 2014.

Complete the constructor, destructor, insert, lookup, and remove
functions (see TODO). You need a C++11 compiler. In total the code
required is less than 50 lines. The main function contains test code
which should execute in less than 2 seconds. */

#include <cassert>
#include <iostream>
#include <cstring>
#include <vector>
#include <algorithm>
#include <chrono>

using namespace std;
using namespace std::chrono;

struct Row {
    /// Attribute a
    uint64_t a;
    /// Attribute b
    uint64_t b;
    /// Attribute c
    uint64_t c;
    /// The next pointer for linking rows that have the same hash value in the hash table
    Row* next;
};

struct Relation {
    /// Number of rows in relation
    uint64_t size;
    /// Size of the hash table, must be a power of two
    uint64_t sizeIndex;
    /// Hash table
    Row** index;

    // Construct a relation
    Relation(uint64_t sizeIndex) : size(0), sizeIndex(sizeIndex) {
        // Check that sizeIndex is a power of two
        assert((sizeIndex & (sizeIndex-1)) == 0);
        index = new Row*[sizeIndex];
    }

    // Destroy relation (free all memory)
    ~Relation() {
        delete[] index;
    }

    // Insert a new row
    void insert(uint64_t a,uint64_t b,uint64_t c) {
        Row* row = new Row {
            a, b, c, nullptr
        };
        uint64_t hash_val = hash(a);

        size++;

        if (index[hash_val]) {
            row->next = index[hash_val];
        }

        index[hash_val] = row;
    }

    /// Find a row using the index
    Row* lookup(uint64_t a) {
        Row* row = index[hash(a)];
        if (!row)
            return nullptr;

        while (row->a != a && row->next)
            row = row->next;

        if (row->a == a)
            return row;
        else
            return nullptr;
    }

    // Remove a row
    void remove(Row* row) {
        uint64_t hash_val = hash(row->a);
        Row* indexptr = index[hash_val];
        while (indexptr && indexptr->next != row) {
            indexptr = indexptr->next;
        }

        size--;

        if (row->next) {
            if (index[hash_val] == row)
                index[hash_val] = row->next;
            else
                indexptr->next = row->next;
        } else {
             if (index[hash_val] == row)
                index[hash_val] = nullptr;
            else
                indexptr->next = nullptr;
        }

        delete row;
    }

    // Computes index into hash table for attribute value a
    uint64_t hash(uint64_t a) {
        return a&(sizeIndex-1);
    }
};

int main() {
    uint64_t n=2500000;
    Relation R(1ull<<20);

    // Random test data
    vector<Row> v;
    for (uint64_t i=0; i<n; i++)
        v.push_back({i,i/3,i/7,nullptr});

    {
        random_shuffle(v.begin(),v.end());
        // Insert test data
        auto start=high_resolution_clock::now();
        for (Row& r : v)
            R.insert(r.a,r.b,r.c);
        cout << "insert " << duration_cast<duration<double>>(high_resolution_clock::now()-start).count() << "s" << endl;
    }

    {
        random_shuffle(v.begin(),v.end());
        // Lookup rows
        auto start=high_resolution_clock::now();
        for (Row& r : v) {
            Row* r2=R.lookup(r.a);
            assert(r2&&(r2->a==r.a));
        }
        cout << "lookup " << duration_cast<duration<double>>(high_resolution_clock::now()-start).count() << "s" << endl;
    }

    {
        auto start=high_resolution_clock::now();
        // Scan all entries and add attribute a
        uint64_t sum=0;
        for (uint64_t i=0; i<R.sizeIndex; i++) {
            Row* r=R.index[i];
            while (r) {
                sum+=r->a;
                r=r->next;
            }
        }
        cout << "scan " << duration_cast<duration<double>>(high_resolution_clock::now()-start).count() << "s" << endl;
        assert(sum==((n*(n-1))/2));
    }

    {
        random_shuffle(v.begin(),v.end());
        // Delete all entries
        auto start=high_resolution_clock::now();
        for (Row& r : v) {
            Row* r2=R.lookup(r.a);
            assert(r2);
            R.remove(r2);
            assert(!R.lookup(r.a));
        }
        cout << "remove " << duration_cast<duration<double>>(high_resolution_clock::now()-start).count() << "s" << endl;
        // Make sure the table is empty
        for (unsigned i=0; i<R.sizeIndex; i++)
            assert(R.index[i]==nullptr);
    }

    return 0;
}

