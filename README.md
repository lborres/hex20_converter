# Hex20 Converter

---
## Understanding the output
Given Sample Output Hex20
`:20004000AF2A0000890000008900000089000000890000000550000089000000890000003C`

From Intel Hex Info
First character ( : )  =  Start of a record
Next two characters  =  Record length (in this example, 20H)
Next four characters =  Load address (in this example, 0040h)

Next two characters  =  Record type (see below)
Remaining characters =  Actual data
Last two characters  =  Checksum (i.e., sum of all bytes checksum = 3C)

HEX record types: 

**00** = Data record                                                              
**01** = End of file record                                                       
**02** = Extended segment address record                                          
**03** = Start segment address record                                             
**04** = Extended linear address record                                           
**05** = Start linear address record 

---
References:
- [Intel Hex](https://www.intel.com/content/www/us/en/support/programmable/articles/000076770.html)

- [Intel Hex Wiki](https://en.wikipedia.org/wiki/Intel_HEX) - Info on Checksum

<!-- [Hex to Bin Converter](https://hex2bin.sourceforge.net/) -->

---
Logging

Oct 3, 2022 - Initial Meeting

Oct 4, 2022 - CLI prep, Parsing .txt file