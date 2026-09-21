Fn 0:2
Ident 3:13 text="categorize"
LParen 13:14
Ident 14:15 text="n"
Colon 15:16
Ident 17:20 text="int"
RParen 20:21
Arrow 22:24
Ident 25:31 text="string"
LBrace 32:33
Newline 33:34
Match 38:43
LParen 44:45
Ident 45:46 text="n"
RParen 46:47
LBrace 48:49
Newline 49:50
Case 58:62
Number 63:64 text="0" suf="" isf=0 int=0
Arrow 65:67
Str 68:74 T("zero")
Newline 74:75
Case 83:87
Number 88:89 text="1" suf="" isf=0 int=1
Arrow 90:92
Str 93:98 T("one")
Newline 98:99
Case 107:111
Number 112:113 text="2" suf="" isf=0 int=2
Arrow 114:116
Str 117:122 T("two")
Newline 122:123
Case 131:135
Ident 136:137 text="_"
Arrow 138:140
Str 141:147 T("many")
Newline 147:148
RBrace 152:153
Newline 153:154
RBrace 154:155
Newline 155:156
Newline 156:157
Fn 157:159
Ident 160:164 text="band"
LParen 164:165
Ident 165:166 text="n"
Colon 166:167
Ident 168:171 text="int"
RParen 171:172
Arrow 173:175
Ident 176:182 text="string"
LBrace 183:184
Newline 184:185
Match 189:194
LParen 195:196
Ident 196:197 text="n"
RParen 197:198
LBrace 199:200
Newline 200:201
Case 209:213
Ident 214:215 text="x"
If 216:218
Ident 219:220 text="x"
Lt 221:222
Number 223:224 text="0" suf="" isf=0 int=0
Arrow 225:227
Str 228:233 T("neg")
Newline 233:234
Case 242:246
Number 247:248 text="0" suf="" isf=0 int=0
Arrow 249:251
Str 252:258 T("zero")
Newline 258:259
Case 267:271
Ident 272:273 text="x"
If 274:276
Ident 277:278 text="x"
Gt 279:280
Number 281:282 text="9" suf="" isf=0 int=9
Arrow 283:285
Str 286:291 T("big")
Newline 291:292
Case 300:304
Ident 305:306 text="_"
Arrow 307:309
Str 310:317 T("small")
Newline 317:318
RBrace 322:323
Newline 323:324
RBrace 324:325
Newline 325:326
Newline 326:327
Fn 327:329
Ident 330:337 text="name_of"
LParen 337:338
Ident 338:339 text="k"
Colon 339:340
Ident 341:347 text="string"
RParen 347:348
Arrow 349:351
Ident 352:358 text="string"
LBrace 359:360
Newline 360:361
Match 365:370
LParen 371:372
Ident 372:373 text="k"
RParen 373:374
LBrace 375:376
Newline 376:377
Case 385:389
Str 390:394 T("up")
Arrow 395:397
Str 398:405 T("north")
Newline 405:406
Case 414:418
Str 419:425 T("down")
Arrow 426:428
Str 429:436 T("south")
Newline 436:437
Case 445:449
Ident 450:451 text="_"
Arrow 452:454
Str 455:458 T("?")
Newline 458:459
RBrace 463:464
Newline 464:465
RBrace 465:466
Newline 466:467
Newline 467:468
Fn 468:470
Ident 471:475 text="flip"
LParen 475:476
Ident 476:477 text="b"
Colon 477:478
Ident 479:483 text="bool"
RParen 483:484
Arrow 485:487
Ident 488:494 text="string"
LBrace 495:496
Newline 496:497
Match 501:506
LParen 507:508
Ident 508:509 text="b"
RParen 509:510
LBrace 511:512
Newline 512:513
Case 521:525
True 526:530
Arrow 531:533
Str 534:539 T("yes")
Newline 539:540
Case 548:552
False 553:558
Arrow 559:561
Str 562:566 T("no")
Newline 566:567
RBrace 571:572
Newline 572:573
RBrace 573:574
Newline 574:575
Newline 575:576
Fn 576:578
Ident 579:584 text="punct"
LParen 584:585
Ident 585:586 text="c"
Colon 586:587
Ident 588:592 text="char"
RParen 592:593
Arrow 594:596
Ident 597:603 text="string"
LBrace 604:605
Newline 605:606
Match 610:615
LParen 616:617
Ident 617:618 text="c"
RParen 618:619
LBrace 620:621
Newline 621:622
Case 630:634
Char 635:638 ch='.' int=46
Arrow 639:641
Str 642:647 T("dot")
Newline 647:648
Case 656:660
Char 661:664 ch=',' int=44
Arrow 665:667
Str 668:675 T("comma")
Newline 675:676
Case 684:688
Ident 689:690 text="_"
Arrow 691:693
Str 694:701 T("other")
Newline 701:702
RBrace 706:707
Newline 707:708
RBrace 708:709
Newline 709:710
Newline 710:711
Fn 711:713
Ident 714:718 text="rate"
LParen 718:719
Ident 719:720 text="x"
Colon 720:721
Ident 722:727 text="float"
RParen 727:728
Arrow 729:731
Ident 732:738 text="string"
LBrace 739:740
Newline 740:741
Match 745:750
LParen 751:752
Ident 752:753 text="x"
RParen 753:754
LBrace 755:756
Newline 756:757
Case 765:769
Number 770:773 text="1.5" suf="" isf=1 int=-
Arrow 774:776
Str 777:793 T("one and a half")
Newline 793:794
Case 802:806
Number 807:810 text="2.0" suf="" isf=1 int=-
Arrow 811:813
Str 814:819 T("two")
Newline 819:820
Case 828:832
Ident 833:834 text="_"
Arrow 835:837
Str 838:845 T("other")
Newline 845:846
RBrace 850:851
Newline 851:852
RBrace 852:853
Newline 853:854
Newline 854:855
Fn 855:857
Ident 858:872 text="unwrap_or_zero"
LParen 872:873
Ident 873:874 text="m"
Colon 874:875
Ident 876:879 text="int"
Question 879:880
RParen 880:881
Arrow 882:884
Ident 885:888 text="int"
LBrace 889:890
Newline 890:891
Match 895:900
LParen 901:902
Ident 902:903 text="m"
RParen 903:904
LBrace 905:906
Newline 906:907
Case 915:919
Ident 920:924 text="some"
LParen 924:925
Ident 925:926 text="v"
RParen 926:927
Arrow 928:930
Ident 931:932 text="v"
Newline 932:933
Case 941:945
None 946:950
Arrow 951:953
Number 954:955 text="0" suf="" isf=0 int=0
Newline 955:956
RBrace 960:961
Newline 961:962
RBrace 962:963
Newline 963:964
Newline 964:965
Fn 965:967
Ident 968:976 text="first_of"
LParen 976:977
Ident 977:978 text="t"
Colon 978:979
Ident 980:986 text="string"
Question 986:987
RParen 987:988
Arrow 989:991
Ident 992:998 text="string"
LBrace 999:1000
Newline 1000:1001
Match 1005:1010
LParen 1011:1012
Ident 1012:1013 text="t"
RParen 1013:1014
LBrace 1015:1016
Newline 1016:1017
Case 1025:1029
Ident 1030:1034 text="some"
LParen 1034:1035
Ident 1035:1036 text="x"
RParen 1036:1037
Arrow 1038:1040
Ident 1041:1042 text="x"
Newline 1042:1043
Case 1051:1055
None 1056:1060
Arrow 1061:1063
Str 1064:1073 T("missing")
Newline 1073:1074
RBrace 1078:1079
Newline 1079:1080
RBrace 1080:1081
Newline 1081:1082
Newline 1082:1083
Fn 1083:1085
Ident 1086:1097 text="opt_or_else"
LParen 1097:1098
Ident 1098:1099 text="m"
Colon 1099:1100
Ident 1101:1104 text="int"
Question 1104:1105
Comma 1105:1106
Ident 1107:1115 text="fallback"
Colon 1115:1116
Ident 1117:1120 text="int"
RParen 1120:1121
Arrow 1122:1124
Ident 1125:1128 text="int"
LBrace 1129:1130
Newline 1130:1131
If 1135:1137
LParen 1138:1139
Let 1139:1142
Ident 1143:1147 text="some"
LParen 1147:1148
Ident 1148:1149 text="v"
RParen 1149:1150
Assign 1151:1152
Ident 1153:1154 text="m"
RParen 1154:1155
LBrace 1156:1157
Newline 1157:1158
Ident 1166:1167 text="v"
Newline 1167:1168
RBrace 1172:1173
Else 1174:1178
LBrace 1179:1180
Newline 1180:1181
Ident 1189:1197 text="fallback"
Newline 1197:1198
RBrace 1202:1203
Newline 1203:1204
RBrace 1204:1205
Newline 1205:1206
Newline 1206:1207
Ident 1207:1211 text="test"
Fn 1212:1214
Ident 1215:1232 text="int_literal_cases"
LParen 1232:1233
RParen 1233:1234
LBrace 1235:1236
Newline 1236:1237
Ident 1241:1247 text="expect"
LParen 1247:1248
Ident 1248:1258 text="categorize"
LParen 1258:1259
Number 1259:1260 text="0" suf="" isf=0 int=0
RParen 1260:1261
RParen 1261:1262
Dot 1262:1263
Ident 1263:1267 text="toBe"
LParen 1267:1268
Str 1268:1274 T("zero")
RParen 1274:1275
Newline 1275:1276
Ident 1280:1286 text="expect"
LParen 1286:1287
Ident 1287:1297 text="categorize"
LParen 1297:1298
Number 1298:1299 text="1" suf="" isf=0 int=1
RParen 1299:1300
RParen 1300:1301
Dot 1301:1302
Ident 1302:1306 text="toBe"
LParen 1306:1307
Str 1307:1312 T("one")
RParen 1312:1313
Newline 1313:1314
Ident 1318:1324 text="expect"
LParen 1324:1325
Ident 1325:1335 text="categorize"
LParen 1335:1336
Number 1336:1337 text="2" suf="" isf=0 int=2
RParen 1337:1338
RParen 1338:1339
Dot 1339:1340
Ident 1340:1344 text="toBe"
LParen 1344:1345
Str 1345:1350 T("two")
RParen 1350:1351
Newline 1351:1352
Ident 1356:1362 text="expect"
LParen 1362:1363
Ident 1363:1373 text="categorize"
LParen 1373:1374
Number 1374:1375 text="7" suf="" isf=0 int=7
RParen 1375:1376
RParen 1376:1377
Dot 1377:1378
Ident 1378:1382 text="toBe"
LParen 1382:1383
Str 1383:1389 T("many")
RParen 1389:1390
Newline 1390:1391
RBrace 1391:1392
Newline 1392:1393
Newline 1393:1394
Ident 1394:1398 text="test"
Fn 1399:1401
Ident 1402:1425 text="int_guards_and_bindings"
LParen 1425:1426
RParen 1426:1427
LBrace 1428:1429
Newline 1429:1430
Ident 1434:1440 text="expect"
LParen 1440:1441
Ident 1441:1445 text="band"
LParen 1445:1446
Minus 1446:1447
Number 1447:1448 text="3" suf="" isf=0 int=3
RParen 1448:1449
RParen 1449:1450
Dot 1450:1451
Ident 1451:1455 text="toBe"
LParen 1455:1456
Str 1456:1461 T("neg")
RParen 1461:1462
Newline 1462:1463
Ident 1467:1473 text="expect"
LParen 1473:1474
Ident 1474:1478 text="band"
LParen 1478:1479
Number 1479:1480 text="0" suf="" isf=0 int=0
RParen 1480:1481
RParen 1481:1482
Dot 1482:1483
Ident 1483:1487 text="toBe"
LParen 1487:1488
Str 1488:1494 T("zero")
RParen 1494:1495
Newline 1495:1496
Ident 1500:1506 text="expect"
LParen 1506:1507
Ident 1507:1511 text="band"
LParen 1511:1512
Number 1512:1513 text="5" suf="" isf=0 int=5
RParen 1513:1514
RParen 1514:1515
Dot 1515:1516
Ident 1516:1520 text="toBe"
LParen 1520:1521
Str 1521:1528 T("small")
RParen 1528:1529
Newline 1529:1530
Ident 1534:1540 text="expect"
LParen 1540:1541
Ident 1541:1545 text="band"
LParen 1545:1546
Number 1546:1548 text="42" suf="" isf=0 int=42
RParen 1548:1549
RParen 1549:1550
Dot 1550:1551
Ident 1551:1555 text="toBe"
LParen 1555:1556
Str 1556:1561 T("big")
RParen 1561:1562
Newline 1562:1563
RBrace 1563:1564
Newline 1564:1565
Newline 1565:1566
Ident 1566:1570 text="test"
Fn 1571:1573
Ident 1574:1594 text="string_literal_cases"
LParen 1594:1595
RParen 1595:1596
LBrace 1597:1598
Newline 1598:1599
Ident 1603:1609 text="expect"
LParen 1609:1610
Ident 1610:1617 text="name_of"
LParen 1617:1618
Str 1618:1622 T("up")
RParen 1622:1623
RParen 1623:1624
Dot 1624:1625
Ident 1625:1629 text="toBe"
LParen 1629:1630
Str 1630:1637 T("north")
RParen 1637:1638
Newline 1638:1639
Ident 1643:1649 text="expect"
LParen 1649:1650
Ident 1650:1657 text="name_of"
LParen 1657:1658
Str 1658:1664 T("down")
RParen 1664:1665
RParen 1665:1666
Dot 1666:1667
Ident 1667:1671 text="toBe"
LParen 1671:1672
Str 1672:1679 T("south")
RParen 1679:1680
Newline 1680:1681
Ident 1685:1691 text="expect"
LParen 1691:1692
Ident 1692:1699 text="name_of"
LParen 1699:1700
Str 1700:1706 T("left")
RParen 1706:1707
RParen 1707:1708
Dot 1708:1709
Ident 1709:1713 text="toBe"
LParen 1713:1714
Str 1714:1717 T("?")
RParen 1717:1718
Newline 1718:1719
RBrace 1719:1720
Newline 1720:1721
Newline 1721:1722
Ident 1722:1726 text="test"
Fn 1727:1729
Ident 1730:1755 text="bool_cases_are_exhaustive"
LParen 1755:1756
RParen 1756:1757
LBrace 1758:1759
Newline 1759:1760
Ident 1764:1770 text="expect"
LParen 1770:1771
Ident 1771:1775 text="flip"
LParen 1775:1776
True 1776:1780
RParen 1780:1781
RParen 1781:1782
Dot 1782:1783
Ident 1783:1787 text="toBe"
LParen 1787:1788
Str 1788:1793 T("yes")
RParen 1793:1794
Newline 1794:1795
Ident 1799:1805 text="expect"
LParen 1805:1806
Ident 1806:1810 text="flip"
LParen 1810:1811
False 1811:1816
RParen 1816:1817
RParen 1817:1818
Dot 1818:1819
Ident 1819:1823 text="toBe"
LParen 1823:1824
Str 1824:1828 T("no")
RParen 1828:1829
Newline 1829:1830
RBrace 1830:1831
Newline 1831:1832
Newline 1832:1833
Ident 1833:1837 text="test"
Fn 1838:1840
Ident 1841:1859 text="char_literal_cases"
LParen 1859:1860
RParen 1860:1861
LBrace 1862:1863
Newline 1863:1864
Ident 1868:1874 text="expect"
LParen 1874:1875
Ident 1875:1880 text="punct"
LParen 1880:1881
Char 1881:1884 ch='.' int=46
RParen 1884:1885
RParen 1885:1886
Dot 1886:1887
Ident 1887:1891 text="toBe"
LParen 1891:1892
Str 1892:1897 T("dot")
RParen 1897:1898
Newline 1898:1899
Ident 1903:1909 text="expect"
LParen 1909:1910
Ident 1910:1915 text="punct"
LParen 1915:1916
Char 1916:1919 ch='x' int=120
RParen 1919:1920
RParen 1920:1921
Dot 1921:1922
Ident 1922:1926 text="toBe"
LParen 1926:1927
Str 1927:1934 T("other")
RParen 1934:1935
Newline 1935:1936
RBrace 1936:1937
Newline 1937:1938
Newline 1938:1939
Ident 1939:1943 text="test"
Fn 1944:1946
Ident 1947:1966 text="float_literal_cases"
LParen 1966:1967
RParen 1967:1968
LBrace 1969:1970
Newline 1970:1971
Ident 1975:1981 text="expect"
LParen 1981:1982
Ident 1982:1986 text="rate"
LParen 1986:1987
Number 1987:1990 text="1.5" suf="" isf=1 int=-
RParen 1990:1991
RParen 1991:1992
Dot 1992:1993
Ident 1993:1997 text="toBe"
LParen 1997:1998
Str 1998:2014 T("one and a half")
RParen 2014:2015
Newline 2015:2016
Ident 2020:2026 text="expect"
LParen 2026:2027
Ident 2027:2031 text="rate"
LParen 2031:2032
Number 2032:2035 text="2.0" suf="" isf=1 int=-
RParen 2035:2036
RParen 2036:2037
Dot 2037:2038
Ident 2038:2042 text="toBe"
LParen 2042:2043
Str 2043:2048 T("two")
RParen 2048:2049
Newline 2049:2050
Ident 2054:2060 text="expect"
LParen 2060:2061
Ident 2061:2065 text="rate"
LParen 2065:2066
Number 2066:2069 text="9.0" suf="" isf=1 int=-
RParen 2069:2070
RParen 2070:2071
Dot 2071:2072
Ident 2072:2076 text="toBe"
LParen 2076:2077
Str 2077:2084 T("other")
RParen 2084:2085
Newline 2085:2086
RBrace 2086:2087
Newline 2087:2088
Newline 2088:2089
Ident 2089:2093 text="test"
Fn 2094:2096
Ident 2097:2116 text="option_some_unwraps"
LParen 2116:2117
RParen 2117:2118
LBrace 2119:2120
Newline 2120:2121
Let 2125:2128
Ident 2129:2130 text="m"
Colon 2130:2131
Ident 2132:2135 text="int"
Question 2135:2136
Assign 2137:2138
Number 2139:2140 text="5" suf="" isf=0 int=5
As 2141:2143
Question 2143:2144
Ident 2145:2148 text="int"
Newline 2148:2149
Ident 2153:2159 text="expect"
LParen 2159:2160
Ident 2160:2174 text="unwrap_or_zero"
LParen 2174:2175
Ident 2175:2176 text="m"
RParen 2176:2177
RParen 2177:2178
Dot 2178:2179
Ident 2179:2183 text="toBe"
LParen 2183:2184
Number 2184:2185 text="5" suf="" isf=0 int=5
RParen 2185:2186
Newline 2186:2187
Let 2191:2194
Ident 2195:2196 text="s"
Colon 2196:2197
Ident 2198:2204 text="string"
Question 2204:2205
Assign 2206:2207
Str 2208:2212 T("hi")
As 2213:2215
Question 2215:2216
Ident 2217:2223 text="string"
Newline 2223:2224
Ident 2228:2234 text="expect"
LParen 2234:2235
Ident 2235:2243 text="first_of"
LParen 2243:2244
Ident 2244:2245 text="s"
RParen 2245:2246
RParen 2246:2247
Dot 2247:2248
Ident 2248:2252 text="toBe"
LParen 2252:2253
Str 2253:2257 T("hi")
RParen 2257:2258
Newline 2258:2259
RBrace 2259:2260
Newline 2260:2261
Newline 2261:2262
Ident 2262:2266 text="test"
Fn 2267:2269
Ident 2270:2298 text="option_none_hits_none_branch"
LParen 2298:2299
RParen 2299:2300
LBrace 2301:2302
Newline 2302:2303
Let 2307:2310
Ident 2311:2312 text="n"
Colon 2312:2313
Ident 2314:2317 text="int"
Question 2317:2318
Assign 2319:2320
None 2321:2325
Newline 2325:2326
Ident 2330:2336 text="expect"
LParen 2336:2337
Ident 2337:2351 text="unwrap_or_zero"
LParen 2351:2352
Ident 2352:2353 text="n"
RParen 2353:2354
RParen 2354:2355
Dot 2355:2356
Ident 2356:2360 text="toBe"
LParen 2360:2361
Number 2361:2362 text="0" suf="" isf=0 int=0
RParen 2362:2363
Newline 2363:2364
RBrace 2364:2365
Newline 2365:2366
Newline 2366:2367
Ident 2367:2371 text="test"
Fn 2372:2374
Ident 2375:2400 text="if_let_some_binds_in_then"
LParen 2400:2401
RParen 2401:2402
LBrace 2403:2404
Newline 2404:2405
Let 2409:2412
Ident 2413:2414 text="m"
Colon 2414:2415
Ident 2416:2419 text="int"
Question 2419:2420
Assign 2421:2422
Number 2423:2424 text="3" suf="" isf=0 int=3
As 2425:2427
Question 2427:2428
Ident 2429:2432 text="int"
Newline 2432:2433
Ident 2437:2443 text="expect"
LParen 2443:2444
Ident 2444:2455 text="opt_or_else"
LParen 2455:2456
Ident 2456:2457 text="m"
Comma 2457:2458
Minus 2459:2460
Number 2460:2461 text="1" suf="" isf=0 int=1
RParen 2461:2462
RParen 2462:2463
Dot 2463:2464
Ident 2464:2468 text="toBe"
LParen 2468:2469
Number 2469:2470 text="3" suf="" isf=0 int=3
RParen 2470:2471
Newline 2471:2472
Let 2476:2479
Ident 2480:2481 text="n"
Colon 2481:2482
Ident 2483:2486 text="int"
Question 2486:2487
Assign 2488:2489
None 2490:2494
Newline 2494:2495
Ident 2499:2505 text="expect"
LParen 2505:2506
Ident 2506:2517 text="opt_or_else"
LParen 2517:2518
Ident 2518:2519 text="n"
Comma 2519:2520
Minus 2521:2522
Number 2522:2523 text="1" suf="" isf=0 int=1
RParen 2523:2524
RParen 2524:2525
Dot 2525:2526
Ident 2526:2530 text="toBe"
LParen 2530:2531
Minus 2531:2532
Number 2532:2533 text="1" suf="" isf=0 int=1
RParen 2533:2534
Newline 2534:2535
RBrace 2535:2536
Newline 2536:2537
Newline 2537:2538
Ident 2538:2542 text="test"
Fn 2543:2545
Ident 2546:2583 text="if_let_plain_binding_is_unconditional"
LParen 2583:2584
RParen 2584:2585
LBrace 2586:2587
Newline 2587:2588
Var 2592:2595
Ident 2596:2600 text="seen"
Assign 2601:2602
Number 2603:2604 text="0" suf="" isf=0 int=0
Newline 2604:2605
If 2609:2611
LParen 2612:2613
Let 2613:2616
Ident 2617:2618 text="x"
Assign 2619:2620
Number 2621:2623 text="42" suf="" isf=0 int=42
RParen 2623:2624
LBrace 2625:2626
Newline 2626:2627
Ident 2635:2639 text="seen"
Assign 2640:2641
Ident 2642:2643 text="x"
Newline 2643:2644
RBrace 2648:2649
Newline 2649:2650
Ident 2654:2660 text="expect"
LParen 2660:2661
Ident 2661:2665 text="seen"
RParen 2665:2666
Dot 2666:2667
Ident 2667:2671 text="toBe"
LParen 2671:2672
Number 2672:2674 text="42" suf="" isf=0 int=42
RParen 2674:2675
Newline 2675:2676
RBrace 2676:2677
Newline 2677:2678
Newline 2678:2679
Ident 2679:2683 text="test"
Fn 2684:2686
Ident 2687:2715 text="if_let_scoping_does_not_leak"
LParen 2715:2716
RParen 2716:2717
LBrace 2718:2719
Newline 2719:2720
Var 2724:2727
Ident 2728:2733 text="broke"
Assign 2734:2735
False 2736:2741
Newline 2741:2742
Let 2746:2749
Ident 2750:2751 text="m"
Colon 2751:2752
Ident 2753:2756 text="int"
Question 2756:2757
Assign 2758:2759
None 2760:2764
Newline 2764:2765
If 2769:2771
LParen 2772:2773
Let 2773:2776
Ident 2777:2781 text="some"
LParen 2781:2782
Ident 2782:2783 text="v"
RParen 2783:2784
Assign 2785:2786
Ident 2787:2788 text="m"
RParen 2788:2789
LBrace 2790:2791
Newline 2791:2792
Ident 2800:2805 text="broke"
Assign 2806:2807
Ident 2808:2809 text="v"
Gt 2810:2811
Number 2812:2813 text="0" suf="" isf=0 int=0
Newline 2813:2814
RBrace 2818:2819
Else 2820:2824
LBrace 2825:2826
Newline 2826:2827
Ident 2835:2840 text="broke"
Assign 2841:2842
False 2843:2848
Newline 2848:2849
RBrace 2853:2854
Newline 2854:2855
Ident 2859:2865 text="expect"
LParen 2865:2866
Ident 2866:2871 text="broke"
RParen 2871:2872
Dot 2872:2873
Ident 2873:2877 text="toBe"
LParen 2877:2878
False 2878:2883
RParen 2883:2884
Newline 2884:2885
RBrace 2885:2886
Eof 2886:2886
