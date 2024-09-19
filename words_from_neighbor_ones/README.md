# Task
* Use the dictionary   unixdict.txt

* Ignore any word in the dictionary whose length is less than 9.


Let's take the words from next characters:
1 <= n < (dictionary length) - 9.
char1 = 1st character of         nth   word.
char2 = 2nd character of  (n+1)th  word.
char3 = 3rd character of  (n+2)th  word.
    ⋮
char9 = 9th character of  (n+8)th  word.


Concatenate (append) the nine characters by:

      newword = char1 + char2 + char3 + ... + char9 

If   newword   is in the dictionary, then show on this page.

Length of  newword = 9

Example output from python code: 

Yes, there are duplicates, the task doesn't say that only unique elements should be present, hence the complete raw list will appear as below :

applicate
architect
astronomy
christine
christoph
committee
committee
committee
committee
committee
composite
constrict
constrict
construct
different
extensive
greenwood
implement
improvise
intercept
interpret
interrupt
interrupt
philosoph
prescript
receptive
telephone
transcend
transcend
transport
transpose