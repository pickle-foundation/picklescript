Interface 0:9
Ident 10:15 text="Shape"
LBrace 16:17
Newline 17:18
Fn 22:24
Ident 25:29 text="area"
LParen 29:30
RParen 30:31
Arrow 32:34
Ident 35:40 text="float"
Newline 40:41
Fn 45:47
Ident 48:53 text="lines"
LParen 53:54
RParen 54:55
Arrow 56:58
Ident 59:62 text="int"
Newline 62:63
RBrace 63:64
Newline 64:65
Newline 65:66
Interface 66:75
Ident 76:81 text="Named"
LBrace 82:83
Newline 83:84
Fn 88:90
Ident 91:96 text="label"
LParen 96:97
RParen 97:98
Arrow 99:101
Ident 102:108 text="string"
Newline 108:109
RBrace 109:110
Newline 110:111
Newline 111:112
Class 112:117
Ident 118:124 text="Circle"
Implements 125:135
Ident 136:141 text="Shape"
LBrace 142:143
Newline 143:144
Var 148:151
Ident 152:158 text="radius"
Colon 158:159
Ident 160:165 text="float"
Newline 165:166
Newline 166:167
Fn 171:173
Ident 174:178 text="area"
LParen 178:179
RParen 179:180
Arrow 181:183
Ident 184:189 text="float"
LBrace 190:191
Newline 191:192
Number 200:210 text="3.14159265" suf="" isf=1 int=-
Star 211:212
This 213:217
Dot 217:218
Ident 218:224 text="radius"
Star 225:226
This 227:231
Dot 231:232
Ident 232:238 text="radius"
Newline 238:239
RBrace 243:244
Newline 244:245
Newline 245:246
Fn 250:252
Ident 253:258 text="lines"
LParen 258:259
RParen 259:260
Arrow 261:263
Ident 264:267 text="int"
LBrace 268:269
Newline 269:270
Number 278:279 text="0" suf="" isf=0 int=0
Newline 279:280
RBrace 284:285
Newline 285:286
RBrace 286:287
Newline 287:288
Newline 288:289
Class 289:294
Ident 295:299 text="Rect"
Implements 300:310
Ident 311:316 text="Shape"
Comma 316:317
Ident 318:323 text="Named"
LBrace 324:325
Newline 325:326
Var 330:333
Ident 334:335 text="w"
Colon 335:336
Ident 337:342 text="float"
Newline 342:343
Var 347:350
Ident 351:352 text="h"
Colon 352:353
Ident 354:359 text="float"
Newline 359:360
Newline 360:361
Fn 365:367
Ident 368:372 text="area"
LParen 372:373
RParen 373:374
Arrow 375:377
Ident 378:383 text="float"
LBrace 384:385
Newline 385:386
This 394:398
Dot 398:399
Ident 399:400 text="w"
Star 401:402
This 403:407
Dot 407:408
Ident 408:409 text="h"
Newline 409:410
RBrace 414:415
Newline 415:416
Newline 416:417
Fn 421:423
Ident 424:429 text="lines"
LParen 429:430
RParen 430:431
Arrow 432:434
Ident 435:438 text="int"
LBrace 439:440
Newline 440:441
Number 449:450 text="4" suf="" isf=0 int=4
Newline 450:451
RBrace 455:456
Newline 456:457
Newline 457:458
Fn 462:464
Ident 465:470 text="label"
LParen 470:471
RParen 471:472
Arrow 473:475
Ident 476:482 text="string"
LBrace 483:484
Newline 484:485
Str 493:499 T("rect")
Newline 499:500
RBrace 504:505
Newline 505:506
RBrace 506:507
Newline 507:508
Newline 508:509
Class 509:514
Ident 515:521 text="Square"
Extends 522:529
Ident 530:534 text="Rect"
LBrace 535:536
Newline 536:537
Override 541:549
Fn 550:552
Ident 553:557 text="area"
LParen 557:558
RParen 558:559
Arrow 560:562
Ident 563:568 text="float"
LBrace 569:570
Newline 570:571
This 579:583
Dot 583:584
Ident 584:585 text="w"
Star 586:587
This 588:592
Dot 592:593
Ident 593:594 text="h"
Newline 594:595
RBrace 599:600
Newline 600:601
Newline 601:602
Override 606:614
Fn 615:617
Ident 618:623 text="label"
LParen 623:624
RParen 624:625
Arrow 626:628
Ident 629:635 text="string"
LBrace 636:637
Newline 637:638
Str 646:654 T("square")
Newline 654:655
RBrace 659:660
Newline 660:661
RBrace 661:662
Newline 662:663
Newline 663:664
Interface 664:673
Ident 674:683 text="Container"
Lt 683:684
Ident 684:685 text="T"
Gt 685:686
LBrace 687:688
Newline 688:689
Fn 693:695
Get 696:699
LParen 699:700
RParen 700:701
Arrow 702:704
Ident 705:706 text="T"
Newline 706:707
Fn 711:713
Ident 714:717 text="put"
LParen 717:718
Ident 718:723 text="value"
Colon 723:724
Ident 725:726 text="T"
RParen 726:727
Newline 727:728
RBrace 728:729
Newline 729:730
Newline 730:731
Class 731:736
Ident 737:743 text="Holder"
Lt 743:744
Ident 744:745 text="T"
Gt 745:746
Implements 747:757
Ident 758:767 text="Container"
Lt 767:768
Ident 768:769 text="T"
Gt 769:770
LBrace 771:772
Newline 772:773
Var 777:780
Ident 781:786 text="value"
Colon 786:787
Ident 788:789 text="T"
Newline 789:790
Newline 790:791
Fn 795:797
Get 798:801
LParen 801:802
RParen 802:803
Arrow 804:806
Ident 807:808 text="T"
LBrace 809:810
Newline 810:811
This 819:823
Dot 823:824
Ident 824:829 text="value"
Newline 829:830
RBrace 834:835
Newline 835:836
Newline 836:837
Fn 841:843
Ident 844:847 text="put"
LParen 847:848
Ident 848:849 text="v"
Colon 849:850
Ident 851:852 text="T"
RParen 852:853
LBrace 854:855
Newline 855:856
This 864:868
Dot 868:869
Ident 869:874 text="value"
Assign 875:876
Ident 877:878 text="v"
Newline 878:879
RBrace 883:884
Newline 884:885
RBrace 885:886
Newline 886:887
Newline 887:888
Fn 888:890
Ident 891:896 text="total"
LParen 896:897
Ident 897:898 text="c"
Colon 898:899
Ident 900:909 text="Container"
Lt 909:910
Ident 910:913 text="int"
Gt 913:914
RParen 914:915
Arrow 916:918
Ident 919:922 text="int"
LBrace 923:924
Newline 924:925
Ident 929:930 text="c"
Dot 930:931
Get 931:934
LParen 934:935
RParen 935:936
Newline 936:937
RBrace 937:938
Newline 938:939
Newline 939:940
Fn 940:942
Ident 943:952 text="shapeArea"
LParen 952:953
Ident 953:954 text="s"
Colon 954:955
Ident 956:961 text="Shape"
RParen 961:962
Arrow 963:965
Ident 966:971 text="float"
LBrace 972:973
Newline 973:974
Ident 978:979 text="s"
Dot 979:980
Ident 980:984 text="area"
LParen 984:985
RParen 985:986
Newline 986:987
RBrace 987:988
Newline 988:989
Newline 989:990
Fn 990:992
Ident 993:1003 text="shapeLines"
LParen 1003:1004
Ident 1004:1005 text="s"
Colon 1005:1006
Ident 1007:1012 text="Shape"
RParen 1012:1013
Arrow 1014:1016
Ident 1017:1020 text="int"
LBrace 1021:1022
Newline 1022:1023
Ident 1027:1028 text="s"
Dot 1028:1029
Ident 1029:1034 text="lines"
LParen 1034:1035
RParen 1035:1036
Newline 1036:1037
RBrace 1037:1038
Newline 1038:1039
Newline 1039:1040
Fn 1040:1042
Ident 1043:1049 text="nameOf"
LParen 1049:1050
Ident 1050:1051 text="n"
Colon 1051:1052
Ident 1053:1058 text="Named"
RParen 1058:1059
Arrow 1060:1062
Ident 1063:1069 text="string"
LBrace 1070:1071
Newline 1071:1072
Ident 1076:1077 text="n"
Dot 1077:1078
Ident 1078:1083 text="label"
LParen 1083:1084
RParen 1084:1085
Newline 1085:1086
RBrace 1086:1087
Newline 1087:1088
Newline 1088:1089
Fn 1089:1091
Ident 1092:1099 text="asShape"
LParen 1099:1100
Ident 1100:1101 text="c"
Colon 1101:1102
Ident 1103:1109 text="Circle"
RParen 1109:1110
Arrow 1111:1113
Ident 1114:1119 text="Shape"
LBrace 1120:1121
Newline 1121:1122
Ident 1126:1127 text="c"
Newline 1127:1128
RBrace 1128:1129
Newline 1129:1130
Newline 1130:1131
Ident 1131:1135 text="test"
Fn 1136:1138
Ident 1139:1164 text="interface_method_dispatch"
LParen 1164:1165
RParen 1165:1166
LBrace 1167:1168
Newline 1168:1169
Ident 1173:1179 text="expect"
LParen 1179:1180
Ident 1180:1189 text="shapeArea"
LParen 1189:1190
Ident 1190:1196 text="Circle"
LParen 1196:1197
Number 1197:1200 text="2.0" suf="" isf=1 int=-
RParen 1200:1201
RParen 1201:1202
RParen 1202:1203
Dot 1203:1204
Ident 1204:1219 text="toBeGreaterThan"
LParen 1219:1220
Number 1220:1225 text="12.56" suf="" isf=1 int=-
RParen 1225:1226
Newline 1226:1227
Ident 1231:1237 text="expect"
LParen 1237:1238
Ident 1238:1247 text="shapeArea"
LParen 1247:1248
Ident 1248:1254 text="Circle"
LParen 1254:1255
Number 1255:1258 text="2.0" suf="" isf=1 int=-
RParen 1258:1259
RParen 1259:1260
RParen 1260:1261
Dot 1261:1262
Ident 1262:1274 text="toBeLessThan"
LParen 1274:1275
Number 1275:1280 text="12.57" suf="" isf=1 int=-
RParen 1280:1281
Newline 1281:1282
Ident 1286:1292 text="expect"
LParen 1292:1293
Ident 1293:1302 text="shapeArea"
LParen 1302:1303
Ident 1303:1307 text="Rect"
LParen 1307:1308
Number 1308:1311 text="3.0" suf="" isf=1 int=-
Comma 1311:1312
Number 1313:1316 text="4.0" suf="" isf=1 int=-
RParen 1316:1317
RParen 1317:1318
RParen 1318:1319
Dot 1319:1320
Ident 1320:1324 text="toBe"
LParen 1324:1325
Number 1325:1329 text="12.0" suf="" isf=1 int=-
RParen 1329:1330
Newline 1330:1331
Ident 1335:1341 text="expect"
LParen 1341:1342
Ident 1342:1352 text="shapeLines"
LParen 1352:1353
Ident 1353:1359 text="Circle"
LParen 1359:1360
Number 1360:1363 text="1.0" suf="" isf=1 int=-
RParen 1363:1364
RParen 1364:1365
RParen 1365:1366
Dot 1366:1367
Ident 1367:1371 text="toBe"
LParen 1371:1372
Number 1372:1373 text="0" suf="" isf=0 int=0
RParen 1373:1374
Newline 1374:1375
Ident 1379:1385 text="expect"
LParen 1385:1386
Ident 1386:1396 text="shapeLines"
LParen 1396:1397
Ident 1397:1401 text="Rect"
LParen 1401:1402
Number 1402:1405 text="3.0" suf="" isf=1 int=-
Comma 1405:1406
Number 1407:1410 text="4.0" suf="" isf=1 int=-
RParen 1410:1411
RParen 1411:1412
RParen 1412:1413
Dot 1413:1414
Ident 1414:1418 text="toBe"
LParen 1418:1419
Number 1419:1420 text="4" suf="" isf=0 int=4
RParen 1420:1421
Newline 1421:1422
Ident 1426:1432 text="expect"
LParen 1432:1433
Ident 1433:1439 text="nameOf"
LParen 1439:1440
Ident 1440:1444 text="Rect"
LParen 1444:1445
Number 1445:1448 text="1.0" suf="" isf=1 int=-
Comma 1448:1449
Number 1450:1453 text="1.0" suf="" isf=1 int=-
RParen 1453:1454
RParen 1454:1455
RParen 1455:1456
Dot 1456:1457
Ident 1457:1461 text="toBe"
LParen 1461:1462
Str 1462:1468 T("rect")
RParen 1468:1469
Newline 1469:1470
RBrace 1470:1471
Newline 1471:1472
Newline 1472:1473
Ident 1473:1477 text="test"
Fn 1478:1480
Ident 1481:1527 text="subclass_override_dispatches_through_interface"
LParen 1527:1528
RParen 1528:1529
LBrace 1530:1531
Newline 1531:1532
Ident 1536:1542 text="expect"
LParen 1542:1543
Ident 1543:1552 text="shapeArea"
LParen 1552:1553
Ident 1553:1559 text="Square"
LParen 1559:1560
Number 1560:1563 text="2.0" suf="" isf=1 int=-
Comma 1563:1564
Number 1565:1568 text="5.0" suf="" isf=1 int=-
RParen 1568:1569
RParen 1569:1570
RParen 1570:1571
Dot 1571:1572
Ident 1572:1576 text="toBe"
LParen 1576:1577
Number 1577:1581 text="10.0" suf="" isf=1 int=-
RParen 1581:1582
Newline 1582:1583
Ident 1587:1593 text="expect"
LParen 1593:1594
Ident 1594:1604 text="shapeLines"
LParen 1604:1605
Ident 1605:1611 text="Square"
LParen 1611:1612
Number 1612:1615 text="2.0" suf="" isf=1 int=-
Comma 1615:1616
Number 1617:1620 text="5.0" suf="" isf=1 int=-
RParen 1620:1621
RParen 1621:1622
RParen 1622:1623
Dot 1623:1624
Ident 1624:1628 text="toBe"
LParen 1628:1629
Number 1629:1630 text="4" suf="" isf=0 int=4
RParen 1630:1631
Newline 1631:1632
Ident 1636:1642 text="expect"
LParen 1642:1643
Ident 1643:1649 text="nameOf"
LParen 1649:1650
Ident 1650:1656 text="Square"
LParen 1656:1657
Number 1657:1660 text="1.0" suf="" isf=1 int=-
Comma 1660:1661
Number 1662:1665 text="1.0" suf="" isf=1 int=-
RParen 1665:1666
RParen 1666:1667
RParen 1667:1668
Dot 1668:1669
Ident 1669:1673 text="toBe"
LParen 1673:1674
Str 1674:1682 T("square")
RParen 1682:1683
Newline 1683:1684
RBrace 1684:1685
Newline 1685:1686
Newline 1686:1687
Ident 1687:1691 text="test"
Fn 1692:1694
Ident 1695:1713 text="interface_is_probe"
LParen 1713:1714
RParen 1714:1715
LBrace 1716:1717
Newline 1717:1718
Ident 1722:1728 text="expect"
LParen 1728:1729
Ident 1729:1735 text="Circle"
LParen 1735:1736
Number 1736:1739 text="1.0" suf="" isf=1 int=-
RParen 1739:1740
Is 1741:1743
Ident 1744:1749 text="Shape"
RParen 1749:1750
Dot 1750:1751
Ident 1751:1755 text="toBe"
LParen 1755:1756
True 1756:1760
RParen 1760:1761
Newline 1761:1762
Ident 1766:1772 text="expect"
LParen 1772:1773
Ident 1773:1777 text="Rect"
LParen 1777:1778
Number 1778:1781 text="1.0" suf="" isf=1 int=-
Comma 1781:1782
Number 1783:1786 text="1.0" suf="" isf=1 int=-
RParen 1786:1787
Is 1788:1790
Ident 1791:1796 text="Shape"
RParen 1796:1797
Dot 1797:1798
Ident 1798:1802 text="toBe"
LParen 1802:1803
True 1803:1807
RParen 1807:1808
Newline 1808:1809
Ident 1813:1819 text="expect"
LParen 1819:1820
Ident 1820:1826 text="Square"
LParen 1826:1827
Number 1827:1830 text="1.0" suf="" isf=1 int=-
Comma 1830:1831
Number 1832:1835 text="1.0" suf="" isf=1 int=-
RParen 1835:1836
Is 1837:1839
Ident 1840:1845 text="Shape"
RParen 1845:1846
Dot 1846:1847
Ident 1847:1851 text="toBe"
LParen 1851:1852
True 1852:1856
RParen 1856:1857
Newline 1857:1858
Ident 1862:1868 text="expect"
LParen 1868:1869
Ident 1869:1873 text="Rect"
LParen 1873:1874
Number 1874:1877 text="1.0" suf="" isf=1 int=-
Comma 1877:1878
Number 1879:1882 text="1.0" suf="" isf=1 int=-
RParen 1882:1883
Is 1884:1886
Ident 1887:1892 text="Named"
RParen 1892:1893
Dot 1893:1894
Ident 1894:1898 text="toBe"
LParen 1898:1899
True 1899:1903
RParen 1903:1904
Newline 1904:1905
Ident 1909:1915 text="expect"
LParen 1915:1916
Ident 1916:1922 text="Square"
LParen 1922:1923
Number 1923:1926 text="1.0" suf="" isf=1 int=-
Comma 1926:1927
Number 1928:1931 text="1.0" suf="" isf=1 int=-
RParen 1931:1932
Is 1933:1935
Ident 1936:1941 text="Named"
RParen 1941:1942
Dot 1942:1943
Ident 1943:1947 text="toBe"
LParen 1947:1948
True 1948:1952
RParen 1952:1953
Newline 1953:1954
RBrace 1954:1955
Newline 1955:1956
Newline 1956:1957
Ident 1957:1961 text="test"
Fn 1962:1964
Ident 1965:1983 text="interface_as_casts"
LParen 1983:1984
RParen 1984:1985
LBrace 1986:1987
Newline 1987:1988
Var 1992:1995
Ident 1996:1997 text="s"
Colon 1997:1998
Ident 1999:2004 text="Shape"
Assign 2005:2006
Ident 2007:2013 text="Circle"
LParen 2013:2014
Number 2014:2017 text="3.0" suf="" isf=1 int=-
RParen 2017:2018
As 2019:2021
Ident 2022:2027 text="Shape"
Newline 2027:2028
Ident 2032:2038 text="expect"
LParen 2038:2039
Ident 2039:2048 text="shapeArea"
LParen 2048:2049
Ident 2049:2050 text="s"
RParen 2050:2051
RParen 2051:2052
Dot 2052:2053
Ident 2053:2068 text="toBeGreaterThan"
LParen 2068:2069
Number 2069:2074 text="28.27" suf="" isf=1 int=-
RParen 2074:2075
Newline 2075:2076
Ident 2080:2086 text="expect"
LParen 2086:2087
Ident 2087:2096 text="shapeArea"
LParen 2096:2097
Ident 2097:2098 text="s"
RParen 2098:2099
RParen 2099:2100
Dot 2100:2101
Ident 2101:2113 text="toBeLessThan"
LParen 2113:2114
Number 2114:2119 text="28.28" suf="" isf=1 int=-
RParen 2119:2120
Newline 2120:2121
Newline 2121:2122
Var 2126:2129
Ident 2130:2134 text="back"
Colon 2134:2135
Ident 2136:2142 text="Circle"
Assign 2143:2144
Ident 2145:2146 text="s"
As 2147:2149
Ident 2150:2156 text="Circle"
Newline 2156:2157
Ident 2161:2167 text="expect"
LParen 2167:2168
Ident 2168:2172 text="back"
Dot 2172:2173
Ident 2173:2179 text="radius"
RParen 2179:2180
Dot 2180:2181
Ident 2181:2185 text="toBe"
LParen 2185:2186
Number 2186:2189 text="3.0" suf="" isf=1 int=-
RParen 2189:2190
Newline 2190:2191
Newline 2191:2192
Var 2196:2199
Ident 2200:2202 text="sq"
Colon 2202:2203
Ident 2204:2209 text="Shape"
Assign 2210:2211
Ident 2212:2218 text="Square"
LParen 2218:2219
Number 2219:2222 text="1.0" suf="" isf=1 int=-
Comma 2222:2223
Number 2224:2227 text="2.0" suf="" isf=1 int=-
RParen 2227:2228
As 2229:2231
Ident 2232:2237 text="Shape"
Newline 2237:2238
Var 2242:2245
Ident 2246:2252 text="sqRect"
Colon 2252:2253
Ident 2254:2258 text="Rect"
Assign 2259:2260
Ident 2261:2263 text="sq"
As 2264:2266
Ident 2267:2271 text="Rect"
Newline 2271:2272
Ident 2276:2282 text="expect"
LParen 2282:2283
Ident 2283:2289 text="sqRect"
Dot 2289:2290
Ident 2290:2291 text="w"
RParen 2291:2292
Dot 2292:2293
Ident 2293:2297 text="toBe"
LParen 2297:2298
Number 2298:2301 text="1.0" suf="" isf=1 int=-
RParen 2301:2302
Newline 2302:2303
RBrace 2303:2304
Newline 2304:2305
Newline 2305:2306
Ident 2306:2310 text="test"
Fn 2311:2313
Ident 2314:2339 text="interface_annotated_value"
LParen 2339:2340
RParen 2340:2341
LBrace 2342:2343
Newline 2343:2344
Let 2348:2351
Ident 2352:2353 text="s"
Colon 2353:2354
Ident 2355:2360 text="Shape"
Assign 2361:2362
Ident 2363:2369 text="Circle"
LParen 2369:2370
Number 2370:2373 text="1.0" suf="" isf=1 int=-
RParen 2373:2374
Newline 2374:2375
Ident 2379:2385 text="expect"
LParen 2385:2386
Ident 2386:2396 text="shapeLines"
LParen 2396:2397
Ident 2397:2398 text="s"
RParen 2398:2399
RParen 2399:2400
Dot 2400:2401
Ident 2401:2405 text="toBe"
LParen 2405:2406
Number 2406:2407 text="0" suf="" isf=0 int=0
RParen 2407:2408
Newline 2408:2409
Ident 2413:2419 text="expect"
LParen 2419:2420
Ident 2420:2421 text="s"
Is 2422:2424
Ident 2425:2431 text="Circle"
RParen 2431:2432
Dot 2432:2433
Ident 2433:2437 text="toBe"
LParen 2437:2438
True 2438:2442
RParen 2442:2443
Newline 2443:2444
Ident 2448:2454 text="expect"
LParen 2454:2455
Ident 2455:2456 text="s"
Is 2457:2459
Ident 2460:2464 text="Rect"
RParen 2464:2465
Dot 2465:2466
Ident 2466:2470 text="toBe"
LParen 2470:2471
False 2471:2476
RParen 2476:2477
Newline 2477:2478
Let 2482:2485
Ident 2486:2487 text="n"
Colon 2487:2488
Ident 2489:2494 text="Named"
Assign 2495:2496
Ident 2497:2503 text="Square"
LParen 2503:2504
Number 2504:2507 text="1.0" suf="" isf=1 int=-
Comma 2507:2508
Number 2509:2512 text="1.0" suf="" isf=1 int=-
RParen 2512:2513
Newline 2513:2514
Ident 2518:2524 text="expect"
LParen 2524:2525
Ident 2525:2526 text="n"
Dot 2526:2527
Ident 2527:2532 text="label"
LParen 2532:2533
RParen 2533:2534
RParen 2534:2535
Dot 2535:2536
Ident 2536:2540 text="toBe"
LParen 2540:2541
Str 2541:2549 T("square")
RParen 2549:2550
Newline 2550:2551
RBrace 2551:2552
Newline 2552:2553
Newline 2553:2554
Ident 2554:2558 text="test"
Fn 2559:2561
Ident 2562:2585 text="interface_valued_return"
LParen 2585:2586
RParen 2586:2587
LBrace 2588:2589
Newline 2589:2590
Var 2594:2597
Ident 2598:2599 text="s"
Colon 2599:2600
Ident 2601:2606 text="Shape"
Assign 2607:2608
Ident 2609:2616 text="asShape"
LParen 2616:2617
Ident 2617:2623 text="Circle"
LParen 2623:2624
Number 2624:2627 text="4.0" suf="" isf=1 int=-
RParen 2627:2628
RParen 2628:2629
Newline 2629:2630
Ident 2634:2640 text="expect"
LParen 2640:2641
Ident 2641:2650 text="shapeArea"
LParen 2650:2651
Ident 2651:2652 text="s"
RParen 2652:2653
RParen 2653:2654
Dot 2654:2655
Ident 2655:2670 text="toBeGreaterThan"
LParen 2670:2671
Number 2671:2676 text="50.26" suf="" isf=1 int=-
RParen 2676:2677
Newline 2677:2678
Ident 2682:2688 text="expect"
LParen 2688:2689
Ident 2689:2698 text="shapeArea"
LParen 2698:2699
Ident 2699:2700 text="s"
RParen 2700:2701
RParen 2701:2702
Dot 2702:2703
Ident 2703:2715 text="toBeLessThan"
LParen 2715:2716
Number 2716:2721 text="50.27" suf="" isf=1 int=-
RParen 2721:2722
Newline 2722:2723
RBrace 2723:2724
Newline 2724:2725
Newline 2725:2726
Ident 2726:2730 text="test"
Fn 2731:2733
Ident 2734:2765 text="generic_interface_instantiation"
LParen 2765:2766
RParen 2766:2767
LBrace 2768:2769
Newline 2769:2770
Var 2774:2777
Ident 2778:2779 text="h"
Assign 2780:2781
Ident 2782:2788 text="Holder"
Lt 2788:2789
Ident 2789:2792 text="int"
Gt 2792:2793
LParen 2793:2794
Number 2794:2795 text="0" suf="" isf=0 int=0
RParen 2795:2796
Newline 2796:2797
Ident 2801:2802 text="h"
Dot 2802:2803
Ident 2803:2806 text="put"
LParen 2806:2807
Number 2807:2808 text="9" suf="" isf=0 int=9
RParen 2808:2809
Newline 2809:2810
Var 2814:2817
Ident 2818:2820 text="ci"
Colon 2820:2821
Ident 2822:2831 text="Container"
Lt 2831:2832
Ident 2832:2835 text="int"
Gt 2835:2836
Assign 2837:2838
Ident 2839:2840 text="h"
Newline 2840:2841
Ident 2845:2851 text="expect"
LParen 2851:2852
Ident 2852:2854 text="ci"
Dot 2854:2855
Get 2855:2858
LParen 2858:2859
RParen 2859:2860
RParen 2860:2861
Dot 2861:2862
Ident 2862:2866 text="toBe"
LParen 2866:2867
Number 2867:2868 text="9" suf="" isf=0 int=9
RParen 2868:2869
Newline 2869:2870
Ident 2874:2880 text="expect"
LParen 2880:2881
Ident 2881:2882 text="h"
Is 2883:2885
Ident 2886:2895 text="Container"
Lt 2895:2896
Ident 2896:2899 text="int"
Gt 2899:2900
RParen 2900:2901
Dot 2901:2902
Ident 2902:2906 text="toBe"
LParen 2906:2907
True 2907:2911
RParen 2911:2912
Newline 2912:2913
Ident 2917:2923 text="expect"
LParen 2923:2924
Ident 2924:2925 text="h"
Is 2926:2928
Ident 2929:2938 text="Container"
Lt 2938:2939
Ident 2939:2945 text="string"
Gt 2945:2946
RParen 2946:2947
Dot 2947:2948
Ident 2948:2952 text="toBe"
LParen 2952:2953
False 2953:2958
RParen 2958:2959
Newline 2959:2960
Ident 2964:2970 text="expect"
LParen 2970:2971
Ident 2971:2976 text="total"
LParen 2976:2977
Ident 2977:2978 text="h"
RParen 2978:2979
RParen 2979:2980
Dot 2980:2981
Ident 2981:2985 text="toBe"
LParen 2985:2986
Number 2986:2987 text="9" suf="" isf=0 int=9
RParen 2987:2988
Newline 2988:2989
Ident 2993:2999 text="expect"
LParen 2999:3000
Ident 3000:3005 text="total"
LParen 3005:3006
Ident 3006:3012 text="Holder"
Lt 3012:3013
Ident 3013:3016 text="int"
Gt 3016:3017
LParen 3017:3018
Number 3018:3020 text="42" suf="" isf=0 int=42
RParen 3020:3021
RParen 3021:3022
RParen 3022:3023
Dot 3023:3024
Ident 3024:3028 text="toBe"
LParen 3028:3029
Number 3029:3031 text="42" suf="" isf=0 int=42
RParen 3031:3032
Newline 3032:3033
RBrace 3033:3034
Newline 3034:3035
Newline 3035:3036
Ident 3036:3040 text="test"
Fn 3041:3043
Ident 3044:3082 text="generic_interface_string_instantiation"
LParen 3082:3083
RParen 3083:3084
LBrace 3085:3086
Newline 3086:3087
Var 3091:3094
Ident 3095:3097 text="hs"
Assign 3098:3099
Ident 3100:3106 text="Holder"
Lt 3106:3107
Ident 3107:3113 text="string"
Gt 3113:3114
LParen 3114:3115
Str 3115:3123 T("pickle")
RParen 3123:3124
Newline 3124:3125
Var 3129:3132
Ident 3133:3135 text="cs"
Colon 3135:3136
Ident 3137:3146 text="Container"
Lt 3146:3147
Ident 3147:3153 text="string"
Gt 3153:3154
Assign 3155:3156
Ident 3157:3159 text="hs"
Newline 3159:3160
Ident 3164:3170 text="expect"
LParen 3170:3171
Ident 3171:3173 text="cs"
Dot 3173:3174
Get 3174:3177
LParen 3177:3178
RParen 3178:3179
RParen 3179:3180
Dot 3180:3181
Ident 3181:3185 text="toBe"
LParen 3185:3186
Str 3186:3194 T("pickle")
RParen 3194:3195
Newline 3195:3196
Ident 3200:3206 text="expect"
LParen 3206:3207
Ident 3207:3209 text="hs"
Is 3210:3212
Ident 3213:3222 text="Container"
Lt 3222:3223
Ident 3223:3229 text="string"
Gt 3229:3230
RParen 3230:3231
Dot 3231:3232
Ident 3232:3236 text="toBe"
LParen 3236:3237
True 3237:3241
RParen 3241:3242
Newline 3242:3243
Ident 3247:3253 text="expect"
LParen 3253:3254
Ident 3254:3256 text="hs"
Is 3257:3259
Ident 3260:3269 text="Container"
Lt 3269:3270
Ident 3270:3273 text="int"
Gt 3273:3274
RParen 3274:3275
Dot 3275:3276
Ident 3276:3280 text="toBe"
LParen 3280:3281
False 3281:3286
RParen 3286:3287
Newline 3287:3288
Ident 3292:3294 text="hs"
Dot 3294:3295
Ident 3295:3298 text="put"
LParen 3298:3299
Str 3299:3307 T("relish")
RParen 3307:3308
Newline 3308:3309
Ident 3313:3319 text="expect"
LParen 3319:3320
Ident 3320:3322 text="cs"
Dot 3322:3323
Get 3323:3326
LParen 3326:3327
RParen 3327:3328
RParen 3328:3329
Dot 3329:3330
Ident 3330:3334 text="toBe"
LParen 3334:3335
Str 3335:3343 T("relish")
RParen 3343:3344
Newline 3344:3345
RBrace 3345:3346
Eof 3346:3346
