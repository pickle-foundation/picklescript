Class 0:5
Ident 6:13 text="Counter"
Implements 14:24
Ident 25:33 text="Iterable"
Lt 33:34
Ident 34:37 text="int"
Gt 37:38
Comma 38:39
Ident 40:48 text="Iterator"
Lt 48:49
Ident 49:52 text="int"
Gt 52:53
LBrace 54:55
Newline 55:56
Var 60:63
Ident 64:65 text="n"
Colon 65:66
Ident 67:70 text="int"
Newline 70:71
Newline 71:72
Fn 76:78
Ident 79:87 text="iterator"
LParen 87:88
RParen 88:89
Arrow 90:92
Ident 93:101 text="Iterator"
Lt 101:102
Ident 102:105 text="int"
Gt 105:106
LBrace 107:108
Newline 108:109
Return 117:123
This 124:128
Newline 128:129
RBrace 133:134
Newline 134:135
Newline 135:136
Fn 140:142
Ident 143:147 text="next"
LParen 147:148
RParen 148:149
Arrow 150:152
Ident 153:156 text="int"
Question 156:157
LBrace 158:159
Newline 159:160
If 168:170
LParen 171:172
Ident 172:173 text="n"
Le 174:176
Number 177:178 text="0" suf="" isf=0 int=0
RParen 178:179
LBrace 180:181
Newline 181:182
Return 194:200
None 201:205
Newline 205:206
RBrace 214:215
Newline 215:216
Let 224:227
Ident 228:229 text="v"
Assign 230:231
Ident 232:233 text="n"
Newline 233:234
Ident 242:243 text="n"
Assign 244:245
Ident 246:247 text="n"
Minus 248:249
Number 250:251 text="1" suf="" isf=0 int=1
Newline 251:252
Return 260:266
Ident 267:268 text="v"
Newline 268:269
RBrace 273:274
Newline 274:275
RBrace 275:276
Newline 276:277
Newline 277:278
Class 278:283
Ident 284:292 text="RangeSeq"
Implements 293:303
Ident 304:312 text="Iterable"
Lt 312:313
Ident 313:316 text="int"
Gt 316:317
LBrace 318:319
Newline 319:320
Var 324:327
Ident 328:330 text="lo"
Colon 330:331
Ident 332:335 text="int"
Newline 335:336
Var 340:343
Ident 344:346 text="hi"
Colon 346:347
Ident 348:351 text="int"
Newline 351:352
Newline 352:353
Fn 357:359
Ident 360:368 text="iterator"
LParen 368:369
RParen 369:370
Arrow 371:373
Ident 374:382 text="Iterator"
Lt 382:383
Ident 383:386 text="int"
Gt 386:387
LBrace 388:389
Newline 389:390
Return 398:404
Ident 405:414 text="RangeIter"
LParen 414:415
This 415:419
Comma 419:420
Ident 421:423 text="lo"
RParen 423:424
Newline 424:425
RBrace 429:430
Newline 430:431
RBrace 431:432
Newline 432:433
Newline 433:434
Class 434:439
Ident 440:449 text="RangeIter"
Implements 450:460
Ident 461:469 text="Iterator"
Lt 469:470
Ident 470:473 text="int"
Gt 473:474
LBrace 475:476
Newline 476:477
Var 481:484
Ident 485:488 text="seq"
Colon 488:489
Ident 490:498 text="RangeSeq"
Newline 498:499
Var 503:506
Ident 507:510 text="cur"
Colon 510:511
Ident 512:515 text="int"
Newline 515:516
Newline 516:517
Fn 521:523
Ident 524:528 text="next"
LParen 528:529
RParen 529:530
Arrow 531:533
Ident 534:537 text="int"
Question 537:538
LBrace 539:540
Newline 540:541
If 549:551
LParen 552:553
Ident 553:556 text="cur"
Ge 557:559
Ident 560:563 text="seq"
Dot 563:564
Ident 564:566 text="hi"
RParen 566:567
LBrace 568:569
Newline 569:570
Return 582:588
None 589:593
Newline 593:594
RBrace 602:603
Newline 603:604
Let 612:615
Ident 616:617 text="v"
Assign 618:619
Ident 620:623 text="cur"
Newline 623:624
Ident 632:635 text="cur"
Assign 636:637
Ident 638:641 text="cur"
Plus 642:643
Number 644:645 text="1" suf="" isf=0 int=1
Newline 645:646
Return 654:660
Ident 661:662 text="v"
Newline 662:663
RBrace 667:668
Newline 668:669
RBrace 669:670
Newline 670:671
Newline 671:672
Class 672:677
Ident 678:686 text="SubRange"
Extends 687:694
Ident 695:703 text="RangeSeq"
LBrace 704:705
RBrace 705:706
Newline 706:707
Newline 707:708
Class 708:713
Ident 714:721 text="WordSeq"
Implements 722:732
Ident 733:741 text="Iterable"
Lt 741:742
Ident 742:748 text="string"
Gt 748:749
LBrace 750:751
Newline 751:752
Var 756:759
Ident 760:765 text="words"
Colon 765:766
Ident 767:771 text="List"
Lt 771:772
Ident 772:778 text="string"
Gt 778:779
Newline 779:780
Newline 780:781
Fn 785:787
Ident 788:796 text="iterator"
LParen 796:797
RParen 797:798
Arrow 799:801
Ident 802:810 text="Iterator"
Lt 810:811
Ident 811:817 text="string"
Gt 817:818
LBrace 819:820
Newline 820:821
Return 829:835
Ident 836:844 text="WordIter"
LParen 844:845
This 845:849
Comma 849:850
Number 851:852 text="0" suf="" isf=0 int=0
RParen 852:853
Newline 853:854
RBrace 858:859
Newline 859:860
RBrace 860:861
Newline 861:862
Newline 862:863
Class 863:868
Ident 869:877 text="WordIter"
Implements 878:888
Ident 889:897 text="Iterator"
Lt 897:898
Ident 898:904 text="string"
Gt 904:905
LBrace 906:907
Newline 907:908
Var 912:915
Ident 916:919 text="seq"
Colon 919:920
Ident 921:928 text="WordSeq"
Newline 928:929
Var 933:936
Ident 937:938 text="i"
Colon 938:939
Ident 940:943 text="int"
Newline 943:944
Newline 944:945
Fn 949:951
Ident 952:956 text="next"
LParen 956:957
RParen 957:958
Arrow 959:961
Ident 962:968 text="string"
Question 968:969
LBrace 970:971
Newline 971:972
If 980:982
LParen 983:984
Ident 984:985 text="i"
Ge 986:988
Number 989:990 text="1" suf="" isf=0 int=1
RParen 990:991
LBrace 992:993
Newline 993:994
Return 1006:1012
None 1013:1017
Newline 1017:1018
RBrace 1026:1027
Newline 1027:1028
Let 1036:1039
Ident 1040:1041 text="w"
Assign 1042:1043
Ident 1044:1047 text="seq"
Dot 1047:1048
Ident 1048:1053 text="words"
LBracket 1053:1054
Number 1054:1055 text="0" suf="" isf=0 int=0
RBracket 1055:1056
Newline 1056:1057
Ident 1065:1066 text="i"
Assign 1067:1068
Ident 1069:1070 text="i"
Plus 1071:1072
Number 1073:1074 text="1" suf="" isf=0 int=1
Newline 1074:1075
Return 1083:1089
Ident 1090:1091 text="w"
Newline 1091:1092
RBrace 1096:1097
Newline 1097:1098
RBrace 1098:1099
Newline 1099:1100
Newline 1100:1101
Class 1101:1106
Ident 1107:1110 text="Bag"
Lt 1110:1111
Ident 1111:1112 text="T"
Gt 1112:1113
Implements 1114:1124
Ident 1125:1133 text="Iterable"
Lt 1133:1134
Ident 1134:1135 text="T"
Gt 1135:1136
LBrace 1137:1138
Newline 1138:1139
Var 1143:1146
Ident 1147:1152 text="items"
Colon 1152:1153
Ident 1154:1158 text="List"
Lt 1158:1159
Ident 1159:1160 text="T"
Gt 1160:1161
Newline 1161:1162
Var 1166:1169
Ident 1170:1171 text="i"
Colon 1171:1172
Ident 1173:1176 text="int"
Newline 1176:1177
Newline 1177:1178
Fn 1182:1184
Ident 1185:1193 text="iterator"
LParen 1193:1194
RParen 1194:1195
Arrow 1196:1198
Ident 1199:1207 text="Iterator"
Lt 1207:1208
Ident 1208:1209 text="T"
Gt 1209:1210
LBrace 1211:1212
Newline 1212:1213
Return 1221:1227
Ident 1228:1235 text="BagIter"
Lt 1235:1236
Ident 1236:1237 text="T"
Gt 1237:1238
LParen 1238:1239
This 1239:1243
Comma 1243:1244
Number 1245:1246 text="0" suf="" isf=0 int=0
RParen 1246:1247
Newline 1247:1248
RBrace 1252:1253
Newline 1253:1254
RBrace 1254:1255
Newline 1255:1256
Newline 1256:1257
Class 1257:1262
Ident 1263:1270 text="BagIter"
Lt 1270:1271
Ident 1271:1272 text="T"
Gt 1272:1273
Implements 1274:1284
Ident 1285:1293 text="Iterator"
Lt 1293:1294
Ident 1294:1295 text="T"
Gt 1295:1296
LBrace 1297:1298
Newline 1298:1299
Var 1303:1306
Ident 1307:1310 text="bag"
Colon 1310:1311
Ident 1312:1315 text="Bag"
Lt 1315:1316
Ident 1316:1317 text="T"
Gt 1317:1318
Newline 1318:1319
Var 1323:1326
Ident 1327:1328 text="i"
Colon 1328:1329
Ident 1330:1333 text="int"
Newline 1333:1334
Newline 1334:1335
Fn 1339:1341
Ident 1342:1346 text="next"
LParen 1346:1347
RParen 1347:1348
Arrow 1349:1351
Ident 1352:1353 text="T"
Question 1353:1354
LBrace 1355:1356
Newline 1356:1357
If 1365:1367
LParen 1368:1369
Ident 1369:1370 text="i"
Ge 1371:1373
Number 1374:1375 text="1" suf="" isf=0 int=1
RParen 1375:1376
LBrace 1377:1378
Newline 1378:1379
Return 1391:1397
None 1398:1402
Newline 1402:1403
RBrace 1411:1412
Newline 1412:1413
Let 1421:1424
Ident 1425:1426 text="v"
Assign 1427:1428
Ident 1429:1432 text="bag"
Dot 1432:1433
Ident 1433:1438 text="items"
LBracket 1438:1439
Number 1439:1440 text="0" suf="" isf=0 int=0
RBracket 1440:1441
Newline 1441:1442
Ident 1450:1451 text="i"
Assign 1452:1453
Ident 1454:1455 text="i"
Plus 1456:1457
Number 1458:1459 text="1" suf="" isf=0 int=1
Newline 1459:1460
Return 1468:1474
Ident 1475:1476 text="v"
Newline 1476:1477
RBrace 1481:1482
Newline 1482:1483
RBrace 1483:1484
Newline 1484:1485
Newline 1485:1486
Fn 1486:1488
Ident 1489:1492 text="sum"
LParen 1492:1493
Ident 1493:1495 text="it"
Colon 1495:1496
Ident 1497:1505 text="Iterable"
Lt 1505:1506
Ident 1506:1509 text="int"
Gt 1509:1510
RParen 1510:1511
Arrow 1512:1514
Ident 1515:1518 text="int"
LBrace 1519:1520
Newline 1520:1521
Var 1525:1528
Ident 1529:1534 text="total"
Assign 1535:1536
Number 1537:1538 text="0" suf="" isf=0 int=0
Newline 1538:1539
For 1543:1546
LParen 1547:1548
Ident 1548:1549 text="x"
In 1550:1552
Ident 1553:1555 text="it"
RParen 1555:1556
LBrace 1557:1558
Newline 1558:1559
Ident 1567:1572 text="total"
Assign 1573:1574
Ident 1575:1580 text="total"
Plus 1581:1582
Ident 1583:1584 text="x"
Newline 1584:1585
RBrace 1589:1590
Newline 1590:1591
Ident 1595:1600 text="total"
Newline 1600:1601
RBrace 1601:1602
Newline 1602:1603
Newline 1603:1604
Fn 1604:1606
Ident 1607:1615 text="countAny"
Lt 1615:1616
Ident 1616:1617 text="T"
Gt 1617:1618
LParen 1618:1619
Ident 1619:1621 text="it"
Colon 1621:1622
Ident 1623:1631 text="Iterable"
Lt 1631:1632
Ident 1632:1633 text="T"
Gt 1633:1634
RParen 1634:1635
Arrow 1636:1638
Ident 1639:1642 text="int"
LBrace 1643:1644
Newline 1644:1645
Var 1649:1652
Ident 1653:1654 text="n"
Assign 1655:1656
Number 1657:1658 text="0" suf="" isf=0 int=0
Newline 1658:1659
For 1663:1666
LParen 1667:1668
Ident 1668:1669 text="x"
In 1670:1672
Ident 1673:1675 text="it"
RParen 1675:1676
LBrace 1677:1678
Newline 1678:1679
Ident 1687:1688 text="n"
Assign 1689:1690
Ident 1691:1692 text="n"
Plus 1693:1694
Number 1695:1696 text="1" suf="" isf=0 int=1
Newline 1696:1697
RBrace 1701:1702
Newline 1702:1703
Ident 1707:1708 text="n"
Newline 1708:1709
RBrace 1709:1710
Newline 1710:1711
Newline 1711:1712
Fn 1712:1714
Ident 1715:1721 text="nextOf"
LParen 1721:1722
Ident 1722:1724 text="it"
Colon 1724:1725
Ident 1726:1734 text="Iterator"
Lt 1734:1735
Ident 1735:1738 text="int"
Gt 1738:1739
RParen 1739:1740
Arrow 1741:1743
Ident 1744:1747 text="int"
Question 1747:1748
LBrace 1749:1750
Newline 1750:1751
Ident 1755:1757 text="it"
Dot 1757:1758
Ident 1758:1762 text="next"
LParen 1762:1763
RParen 1763:1764
Newline 1764:1765
RBrace 1765:1766
Newline 1766:1767
Newline 1767:1768
Ident 1768:1772 text="test"
Fn 1773:1775
Ident 1776:1817 text="protocol_for_in_over_self_iterating_class"
LParen 1817:1818
RParen 1818:1819
LBrace 1820:1821
Newline 1821:1822
Ident 1826:1832 text="expect"
LParen 1832:1833
Ident 1833:1836 text="sum"
LParen 1836:1837
Ident 1837:1844 text="Counter"
LParen 1844:1845
Number 1845:1846 text="3" suf="" isf=0 int=3
RParen 1846:1847
RParen 1847:1848
RParen 1848:1849
Dot 1849:1850
Ident 1850:1854 text="toBe"
LParen 1854:1855
Number 1855:1856 text="6" suf="" isf=0 int=6
RParen 1856:1857
Newline 1857:1858
Ident 1862:1868 text="expect"
LParen 1868:1869
Ident 1869:1872 text="sum"
LParen 1872:1873
Ident 1873:1880 text="Counter"
LParen 1880:1881
Number 1881:1882 text="1" suf="" isf=0 int=1
RParen 1882:1883
RParen 1883:1884
RParen 1884:1885
Dot 1885:1886
Ident 1886:1890 text="toBe"
LParen 1890:1891
Number 1891:1892 text="1" suf="" isf=0 int=1
RParen 1892:1893
Newline 1893:1894
RBrace 1894:1895
Newline 1895:1896
Newline 1896:1897
Ident 1897:1901 text="test"
Fn 1902:1904
Ident 1905:1943 text="protocol_for_in_over_separate_iterator"
LParen 1943:1944
RParen 1944:1945
LBrace 1946:1947
Newline 1947:1948
Ident 1952:1958 text="expect"
LParen 1958:1959
Ident 1959:1962 text="sum"
LParen 1962:1963
Ident 1963:1971 text="RangeSeq"
LParen 1971:1972
Number 1972:1973 text="3" suf="" isf=0 int=3
Comma 1973:1974
Number 1975:1976 text="8" suf="" isf=0 int=8
RParen 1976:1977
RParen 1977:1978
RParen 1978:1979
Dot 1979:1980
Ident 1980:1984 text="toBe"
LParen 1984:1985
Number 1985:1987 text="25" suf="" isf=0 int=25
RParen 1987:1988
Newline 1988:1989
Ident 1993:1999 text="expect"
LParen 1999:2000
Ident 2000:2003 text="sum"
LParen 2003:2004
Ident 2004:2012 text="RangeSeq"
LParen 2012:2013
Number 2013:2014 text="0" suf="" isf=0 int=0
Comma 2014:2015
Number 2016:2017 text="1" suf="" isf=0 int=1
RParen 2017:2018
RParen 2018:2019
RParen 2019:2020
Dot 2020:2021
Ident 2021:2025 text="toBe"
LParen 2025:2026
Number 2026:2027 text="0" suf="" isf=0 int=0
RParen 2027:2028
Newline 2028:2029
RBrace 2029:2030
Newline 2030:2031
Newline 2031:2032
Ident 2032:2036 text="test"
Fn 2037:2039
Ident 2040:2070 text="protocol_for_in_empty_sequence"
LParen 2070:2071
RParen 2071:2072
LBrace 2073:2074
Newline 2074:2075
Ident 2079:2085 text="expect"
LParen 2085:2086
Ident 2086:2089 text="sum"
LParen 2089:2090
Ident 2090:2097 text="Counter"
LParen 2097:2098
Number 2098:2099 text="0" suf="" isf=0 int=0
RParen 2099:2100
RParen 2100:2101
RParen 2101:2102
Dot 2102:2103
Ident 2103:2107 text="toBe"
LParen 2107:2108
Number 2108:2109 text="0" suf="" isf=0 int=0
RParen 2109:2110
Newline 2110:2111
Ident 2115:2121 text="expect"
LParen 2121:2122
Ident 2122:2125 text="sum"
LParen 2125:2126
Ident 2126:2134 text="RangeSeq"
LParen 2134:2135
Number 2135:2136 text="5" suf="" isf=0 int=5
Comma 2136:2137
Number 2138:2139 text="5" suf="" isf=0 int=5
RParen 2139:2140
RParen 2140:2141
RParen 2141:2142
Dot 2142:2143
Ident 2143:2147 text="toBe"
LParen 2147:2148
Number 2148:2149 text="0" suf="" isf=0 int=0
RParen 2149:2150
Newline 2150:2151
RBrace 2151:2152
Newline 2152:2153
Newline 2153:2154
Ident 2154:2158 text="test"
Fn 2159:2161
Ident 2162:2202 text="protocol_for_in_interface_typed_variable"
LParen 2202:2203
RParen 2203:2204
LBrace 2205:2206
Newline 2206:2207
Let 2211:2214
Ident 2215:2217 text="it"
Colon 2217:2218
Ident 2219:2227 text="Iterable"
Lt 2227:2228
Ident 2228:2231 text="int"
Gt 2231:2232
Assign 2233:2234
Ident 2235:2242 text="Counter"
LParen 2242:2243
Number 2243:2244 text="4" suf="" isf=0 int=4
RParen 2244:2245
Newline 2245:2246
Ident 2250:2256 text="expect"
LParen 2256:2257
Ident 2257:2260 text="sum"
LParen 2260:2261
Ident 2261:2263 text="it"
RParen 2263:2264
RParen 2264:2265
Dot 2265:2266
Ident 2266:2270 text="toBe"
LParen 2270:2271
Number 2271:2273 text="10" suf="" isf=0 int=10
RParen 2273:2274
Newline 2274:2275
RBrace 2275:2276
Newline 2276:2277
Newline 2277:2278
Ident 2278:2282 text="test"
Fn 2283:2285
Ident 2286:2317 text="protocol_for_in_string_elements"
LParen 2317:2318
RParen 2318:2319
LBrace 2320:2321
Newline 2321:2322
Let 2326:2329
Ident 2330:2332 text="it"
Colon 2332:2333
Ident 2334:2342 text="Iterable"
Lt 2342:2343
Ident 2343:2349 text="string"
Gt 2349:2350
Assign 2351:2352
Ident 2353:2360 text="WordSeq"
LParen 2360:2361
LBracket 2361:2362
Str 2362:2370 T("pickle")
RBracket 2370:2371
RParen 2371:2372
Newline 2372:2373
Var 2377:2380
Ident 2381:2382 text="n"
Assign 2383:2384
Number 2385:2386 text="0" suf="" isf=0 int=0
Newline 2386:2387
For 2391:2394
LParen 2395:2396
Ident 2396:2397 text="w"
In 2398:2400
Ident 2401:2403 text="it"
RParen 2403:2404
LBrace 2405:2406
Newline 2406:2407
Ident 2415:2416 text="n"
Assign 2417:2418
Ident 2419:2420 text="n"
Plus 2421:2422
Number 2423:2424 text="1" suf="" isf=0 int=1
Newline 2424:2425
RBrace 2429:2430
Newline 2430:2431
Ident 2435:2441 text="expect"
LParen 2441:2442
Ident 2442:2443 text="n"
RParen 2443:2444
Dot 2444:2445
Ident 2445:2449 text="toBe"
LParen 2449:2450
Number 2450:2451 text="1" suf="" isf=0 int=1
RParen 2451:2452
Newline 2452:2453
RBrace 2453:2454
Newline 2454:2455
Newline 2455:2456
Ident 2456:2460 text="test"
Fn 2461:2463
Ident 2464:2493 text="protocol_for_in_generic_class"
LParen 2493:2494
RParen 2494:2495
LBrace 2496:2497
Newline 2497:2498
Var 2502:2505
Ident 2506:2507 text="b"
Colon 2507:2508
Ident 2509:2512 text="Bag"
Lt 2512:2513
Ident 2513:2516 text="int"
Gt 2516:2517
Assign 2518:2519
Ident 2520:2523 text="Bag"
Lt 2523:2524
Ident 2524:2527 text="int"
Gt 2527:2528
LParen 2528:2529
LBracket 2529:2530
Number 2530:2531 text="7" suf="" isf=0 int=7
RBracket 2531:2532
Comma 2532:2533
Number 2534:2535 text="0" suf="" isf=0 int=0
RParen 2535:2536
Newline 2536:2537
Ident 2541:2547 text="expect"
LParen 2547:2548
Ident 2548:2556 text="countAny"
Lt 2556:2557
Ident 2557:2560 text="int"
Gt 2560:2561
LParen 2561:2562
Ident 2562:2563 text="b"
RParen 2563:2564
RParen 2564:2565
Dot 2565:2566
Ident 2566:2570 text="toBe"
LParen 2570:2571
Number 2571:2572 text="1" suf="" isf=0 int=1
RParen 2572:2573
Newline 2573:2574
RBrace 2574:2575
Newline 2575:2576
Newline 2576:2577
Ident 2577:2581 text="test"
Fn 2582:2584
Ident 2585:2617 text="protocol_for_in_generic_function"
LParen 2617:2618
RParen 2618:2619
LBrace 2620:2621
Newline 2621:2622
Ident 2626:2632 text="expect"
LParen 2632:2633
Ident 2633:2641 text="countAny"
Lt 2641:2642
Ident 2642:2645 text="int"
Gt 2645:2646
LParen 2646:2647
Ident 2647:2654 text="Counter"
LParen 2654:2655
Number 2655:2656 text="3" suf="" isf=0 int=3
RParen 2656:2657
RParen 2657:2658
RParen 2658:2659
Dot 2659:2660
Ident 2660:2664 text="toBe"
LParen 2664:2665
Number 2665:2666 text="3" suf="" isf=0 int=3
RParen 2666:2667
Newline 2667:2668
Ident 2672:2678 text="expect"
LParen 2678:2679
Ident 2679:2687 text="countAny"
Lt 2687:2688
Ident 2688:2691 text="int"
Gt 2691:2692
LParen 2692:2693
Ident 2693:2701 text="RangeSeq"
LParen 2701:2702
Number 2702:2703 text="2" suf="" isf=0 int=2
Comma 2703:2704
Number 2705:2706 text="6" suf="" isf=0 int=6
RParen 2706:2707
RParen 2707:2708
RParen 2708:2709
Dot 2709:2710
Ident 2710:2714 text="toBe"
LParen 2714:2715
Number 2715:2716 text="4" suf="" isf=0 int=4
RParen 2716:2717
Newline 2717:2718
RBrace 2718:2719
Newline 2719:2720
Newline 2720:2721
Ident 2721:2725 text="test"
Fn 2726:2728
Ident 2729:2764 text="protocol_for_in_inherits_implements"
LParen 2764:2765
RParen 2765:2766
LBrace 2767:2768
Newline 2768:2769
Ident 2773:2779 text="expect"
LParen 2779:2780
Ident 2780:2783 text="sum"
LParen 2783:2784
Ident 2784:2792 text="SubRange"
LParen 2792:2793
Number 2793:2794 text="3" suf="" isf=0 int=3
Comma 2794:2795
Number 2796:2797 text="7" suf="" isf=0 int=7
RParen 2797:2798
RParen 2798:2799
RParen 2799:2800
Dot 2800:2801
Ident 2801:2805 text="toBe"
LParen 2805:2806
Number 2806:2808 text="18" suf="" isf=0 int=18
RParen 2808:2809
Newline 2809:2810
RBrace 2810:2811
Newline 2811:2812
Newline 2812:2813
Ident 2813:2817 text="test"
Fn 2818:2820
Ident 2821:2840 text="protocol_loop_break"
LParen 2840:2841
RParen 2841:2842
LBrace 2843:2844
Newline 2844:2845
Var 2849:2852
Ident 2853:2858 text="total"
Assign 2859:2860
Number 2861:2862 text="0" suf="" isf=0 int=0
Newline 2862:2863
For 2867:2870
LParen 2871:2872
Ident 2872:2873 text="x"
In 2874:2876
Ident 2877:2885 text="RangeSeq"
LParen 2885:2886
Number 2886:2887 text="0" suf="" isf=0 int=0
Comma 2887:2888
Number 2889:2891 text="10" suf="" isf=0 int=10
RParen 2891:2892
RParen 2892:2893
LBrace 2894:2895
Newline 2895:2896
If 2904:2906
LParen 2907:2908
Ident 2908:2909 text="x"
Ge 2910:2912
Number 2913:2914 text="4" suf="" isf=0 int=4
RParen 2914:2915
LBrace 2916:2917
Newline 2917:2918
Break 2930:2935
Newline 2935:2936
RBrace 2944:2945
Newline 2945:2946
Ident 2954:2959 text="total"
Assign 2960:2961
Ident 2962:2967 text="total"
Plus 2968:2969
Ident 2970:2971 text="x"
Newline 2971:2972
RBrace 2976:2977
Newline 2977:2978
Ident 2982:2988 text="expect"
LParen 2988:2989
Ident 2989:2994 text="total"
RParen 2994:2995
Dot 2995:2996
Ident 2996:3000 text="toBe"
LParen 3000:3001
Number 3001:3002 text="6" suf="" isf=0 int=6
RParen 3002:3003
Newline 3003:3004
RBrace 3004:3005
Newline 3005:3006
Newline 3006:3007
Ident 3007:3011 text="test"
Fn 3012:3014
Ident 3015:3037 text="protocol_loop_continue"
LParen 3037:3038
RParen 3038:3039
LBrace 3040:3041
Newline 3041:3042
Var 3046:3049
Ident 3050:3055 text="count"
Assign 3056:3057
Number 3058:3059 text="0" suf="" isf=0 int=0
Newline 3059:3060
Var 3064:3067
Ident 3068:3076 text="sum_even"
Assign 3077:3078
Number 3079:3080 text="0" suf="" isf=0 int=0
Newline 3080:3081
For 3085:3088
LParen 3089:3090
Ident 3090:3091 text="x"
In 3092:3094
Ident 3095:3103 text="RangeSeq"
LParen 3103:3104
Number 3104:3105 text="0" suf="" isf=0 int=0
Comma 3105:3106
Number 3107:3109 text="10" suf="" isf=0 int=10
RParen 3109:3110
RParen 3110:3111
LBrace 3112:3113
Newline 3113:3114
Ident 3122:3127 text="count"
Assign 3128:3129
Ident 3130:3135 text="count"
Plus 3136:3137
Number 3138:3139 text="1" suf="" isf=0 int=1
Newline 3139:3140
If 3148:3150
LParen 3151:3152
Ident 3152:3153 text="x"
Percent 3154:3155
Number 3156:3157 text="2" suf="" isf=0 int=2
EqEq 3158:3160
Number 3161:3162 text="1" suf="" isf=0 int=1
RParen 3162:3163
LBrace 3164:3165
Newline 3165:3166
Continue 3178:3186
Newline 3186:3187
RBrace 3195:3196
Newline 3196:3197
Ident 3205:3213 text="sum_even"
Assign 3214:3215
Ident 3216:3224 text="sum_even"
Plus 3225:3226
Ident 3227:3228 text="x"
Newline 3228:3229
RBrace 3233:3234
Newline 3234:3235
Ident 3239:3245 text="expect"
LParen 3245:3246
Ident 3246:3251 text="count"
RParen 3251:3252
Dot 3252:3253
Ident 3253:3257 text="toBe"
LParen 3257:3258
Number 3258:3260 text="10" suf="" isf=0 int=10
RParen 3260:3261
Newline 3261:3262
Ident 3266:3272 text="expect"
LParen 3272:3273
Ident 3273:3281 text="sum_even"
RParen 3281:3282
Dot 3282:3283
Ident 3283:3287 text="toBe"
LParen 3287:3288
Number 3288:3290 text="20" suf="" isf=0 int=20
RParen 3290:3291
Newline 3291:3292
RBrace 3292:3293
Newline 3293:3294
Newline 3294:3295
Ident 3295:3299 text="test"
Fn 3300:3302
Ident 3303:3337 text="protocol_reiterates_fresh_iterator"
LParen 3337:3338
RParen 3338:3339
LBrace 3340:3341
Newline 3341:3342
Ident 3346:3352 text="expect"
LParen 3352:3353
Ident 3353:3356 text="sum"
LParen 3356:3357
Ident 3357:3365 text="RangeSeq"
LParen 3365:3366
Number 3366:3367 text="1" suf="" isf=0 int=1
Comma 3367:3368
Number 3369:3370 text="5" suf="" isf=0 int=5
RParen 3370:3371
RParen 3371:3372
RParen 3372:3373
Dot 3373:3374
Ident 3374:3378 text="toBe"
LParen 3378:3379
Number 3379:3381 text="10" suf="" isf=0 int=10
RParen 3381:3382
Newline 3382:3383
Ident 3387:3393 text="expect"
LParen 3393:3394
Ident 3394:3397 text="sum"
LParen 3397:3398
Ident 3398:3406 text="RangeSeq"
LParen 3406:3407
Number 3407:3408 text="1" suf="" isf=0 int=1
Comma 3408:3409
Number 3410:3411 text="5" suf="" isf=0 int=5
RParen 3411:3412
RParen 3412:3413
RParen 3413:3414
Dot 3414:3415
Ident 3415:3419 text="toBe"
LParen 3419:3420
Number 3420:3422 text="10" suf="" isf=0 int=10
RParen 3422:3423
Newline 3423:3424
RBrace 3424:3425
Newline 3425:3426
Newline 3426:3427
Ident 3427:3431 text="test"
Fn 3432:3434
Ident 3435:3466 text="iterator_interface_direct_calls"
LParen 3466:3467
RParen 3467:3468
LBrace 3469:3470
Newline 3470:3471
Var 3475:3478
Ident 3479:3481 text="it"
Colon 3481:3482
Ident 3483:3491 text="Iterator"
Lt 3491:3492
Ident 3492:3495 text="int"
Gt 3495:3496
Assign 3497:3498
Ident 3499:3507 text="RangeSeq"
LParen 3507:3508
Number 3508:3509 text="1" suf="" isf=0 int=1
Comma 3509:3510
Number 3511:3512 text="4" suf="" isf=0 int=4
RParen 3512:3513
Dot 3513:3514
Ident 3514:3522 text="iterator"
LParen 3522:3523
RParen 3523:3524
Newline 3524:3525
Let 3529:3532
Ident 3533:3534 text="a"
Assign 3535:3536
Ident 3537:3543 text="nextOf"
LParen 3543:3544
Ident 3544:3546 text="it"
RParen 3546:3547
Newline 3547:3548
Let 3552:3555
Ident 3556:3557 text="b"
Assign 3558:3559
Ident 3560:3566 text="nextOf"
LParen 3566:3567
Ident 3567:3569 text="it"
RParen 3569:3570
Newline 3570:3571
Let 3575:3578
Ident 3579:3580 text="c"
Assign 3581:3582
Ident 3583:3589 text="nextOf"
LParen 3589:3590
Ident 3590:3592 text="it"
RParen 3592:3593
Newline 3593:3594
Let 3598:3601
Ident 3602:3603 text="d"
Assign 3604:3605
Ident 3606:3612 text="nextOf"
LParen 3612:3613
Ident 3613:3615 text="it"
RParen 3615:3616
Newline 3616:3617
If 3621:3623
LParen 3624:3625
Let 3625:3628
Ident 3629:3633 text="some"
LParen 3633:3634
Ident 3634:3635 text="x"
RParen 3635:3636
Assign 3637:3638
Ident 3639:3640 text="a"
RParen 3640:3641
LBrace 3642:3643
Newline 3643:3644
Ident 3652:3658 text="expect"
LParen 3658:3659
Ident 3659:3660 text="x"
RParen 3660:3661
Dot 3661:3662
Ident 3662:3666 text="toBe"
LParen 3666:3667
Number 3667:3668 text="1" suf="" isf=0 int=1
RParen 3668:3669
Newline 3669:3670
RBrace 3674:3675
Else 3676:3680
LBrace 3681:3682
Newline 3682:3683
Ident 3691:3697 text="expect"
LParen 3697:3698
True 3698:3702
RParen 3702:3703
Dot 3703:3704
Ident 3704:3708 text="toBe"
LParen 3708:3709
False 3709:3714
RParen 3714:3715
Newline 3715:3716
RBrace 3720:3721
Newline 3721:3722
If 3726:3728
LParen 3729:3730
Let 3730:3733
Ident 3734:3738 text="some"
LParen 3738:3739
Ident 3739:3740 text="x"
RParen 3740:3741
Assign 3742:3743
Ident 3744:3745 text="b"
RParen 3745:3746
LBrace 3747:3748
Newline 3748:3749
Ident 3757:3763 text="expect"
LParen 3763:3764
Ident 3764:3765 text="x"
RParen 3765:3766
Dot 3766:3767
Ident 3767:3771 text="toBe"
LParen 3771:3772
Number 3772:3773 text="2" suf="" isf=0 int=2
RParen 3773:3774
Newline 3774:3775
RBrace 3779:3780
Else 3781:3785
LBrace 3786:3787
Newline 3787:3788
Ident 3796:3802 text="expect"
LParen 3802:3803
True 3803:3807
RParen 3807:3808
Dot 3808:3809
Ident 3809:3813 text="toBe"
LParen 3813:3814
False 3814:3819
RParen 3819:3820
Newline 3820:3821
RBrace 3825:3826
Newline 3826:3827
If 3831:3833
LParen 3834:3835
Let 3835:3838
Ident 3839:3843 text="some"
LParen 3843:3844
Ident 3844:3845 text="x"
RParen 3845:3846
Assign 3847:3848
Ident 3849:3850 text="c"
RParen 3850:3851
LBrace 3852:3853
Newline 3853:3854
Ident 3862:3868 text="expect"
LParen 3868:3869
Ident 3869:3870 text="x"
RParen 3870:3871
Dot 3871:3872
Ident 3872:3876 text="toBe"
LParen 3876:3877
Number 3877:3878 text="3" suf="" isf=0 int=3
RParen 3878:3879
Newline 3879:3880
RBrace 3884:3885
Else 3886:3890
LBrace 3891:3892
Newline 3892:3893
Ident 3901:3907 text="expect"
LParen 3907:3908
True 3908:3912
RParen 3912:3913
Dot 3913:3914
Ident 3914:3918 text="toBe"
LParen 3918:3919
False 3919:3924
RParen 3924:3925
Newline 3925:3926
RBrace 3930:3931
Newline 3931:3932
If 3936:3938
LParen 3939:3940
Let 3940:3943
Ident 3944:3948 text="some"
LParen 3948:3949
Ident 3949:3951 text="_x"
RParen 3951:3952
Assign 3953:3954
Ident 3955:3956 text="d"
RParen 3956:3957
LBrace 3958:3959
Newline 3959:3960
Ident 3968:3974 text="expect"
LParen 3974:3975
True 3975:3979
RParen 3979:3980
Dot 3980:3981
Ident 3981:3985 text="toBe"
LParen 3985:3986
False 3986:3991
RParen 3991:3992
Newline 3992:3993
RBrace 3997:3998
Else 3999:4003
LBrace 4004:4005
Newline 4005:4006
Ident 4014:4020 text="expect"
LParen 4020:4021
Number 4021:4022 text="1" suf="" isf=0 int=1
RParen 4022:4023
Dot 4023:4024
Ident 4024:4028 text="toBe"
LParen 4028:4029
Number 4029:4030 text="1" suf="" isf=0 int=1
RParen 4030:4031
Newline 4031:4032
RBrace 4036:4037
Newline 4037:4038
RBrace 4038:4039
Newline 4039:4040
Newline 4040:4041
Ident 4041:4045 text="test"
Fn 4046:4048
Ident 4049:4084 text="iterable_interface_returns_iterator"
LParen 4084:4085
RParen 4085:4086
LBrace 4087:4088
Newline 4088:4089
Var 4093:4096
Ident 4097:4098 text="s"
Colon 4098:4099
Ident 4100:4108 text="Iterable"
Lt 4108:4109
Ident 4109:4112 text="int"
Gt 4112:4113
Assign 4114:4115
Ident 4116:4124 text="RangeSeq"
LParen 4124:4125
Number 4125:4126 text="2" suf="" isf=0 int=2
Comma 4126:4127
Number 4128:4129 text="5" suf="" isf=0 int=5
RParen 4129:4130
Newline 4130:4131
Var 4135:4138
Ident 4139:4144 text="total"
Assign 4145:4146
Number 4147:4148 text="0" suf="" isf=0 int=0
Newline 4148:4149
For 4153:4156
LParen 4157:4158
Ident 4158:4159 text="x"
In 4160:4162
Ident 4163:4164 text="s"
RParen 4164:4165
LBrace 4166:4167
Newline 4167:4168
Ident 4176:4181 text="total"
Assign 4182:4183
Ident 4184:4189 text="total"
Plus 4190:4191
Ident 4192:4193 text="x"
Newline 4193:4194
RBrace 4198:4199
Newline 4199:4200
Ident 4204:4210 text="expect"
LParen 4210:4211
Ident 4211:4216 text="total"
RParen 4216:4217
Dot 4217:4218
Ident 4218:4222 text="toBe"
LParen 4222:4223
Number 4223:4224 text="9" suf="" isf=0 int=9
RParen 4224:4225
Newline 4225:4226
RBrace 4226:4227
Eof 4227:4227
