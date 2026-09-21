Class 0:5
Ident 6:13 text="BaseBox"
LBrace 14:15
Newline 15:16
Var 20:23
Ident 24:25 text="g"
Colon 25:26
Ident 27:30 text="int"
Assign 31:32
Number 33:36 text="100" suf="" isf=0 int=100
Newline 36:37
Newline 37:38
Constructor 42:53
LParen 53:54
Ident 54:55 text="g"
Colon 55:56
Ident 57:60 text="int"
RParen 60:61
LBrace 62:63
Newline 63:64
This 72:76
Dot 76:77
Ident 77:78 text="g"
Assign 79:80
This 81:85
Dot 85:86
Ident 86:87 text="g"
Plus 88:89
Ident 90:91 text="g"
Newline 91:92
RBrace 96:97
Newline 97:98
RBrace 98:99
Newline 99:100
Newline 100:101
Class 101:106
Ident 107:113 text="MidBox"
Extends 114:121
Ident 122:129 text="BaseBox"
LBrace 130:131
Newline 131:132
Var 136:139
Ident 140:141 text="m"
Colon 141:142
Ident 143:146 text="int"
Newline 146:147
Newline 147:148
Constructor 152:163
LParen 163:164
Ident 164:165 text="m"
Colon 165:166
Ident 167:170 text="int"
RParen 170:171
LBrace 172:173
Newline 173:174
Super 182:187
LParen 187:188
Ident 188:189 text="m"
Star 190:191
Number 192:193 text="2" suf="" isf=0 int=2
RParen 193:194
Newline 194:195
This 203:207
Dot 207:208
Ident 208:209 text="m"
Assign 210:211
Ident 212:213 text="m"
Newline 213:214
RBrace 218:219
Newline 219:220
RBrace 220:221
Newline 221:222
Newline 222:223
Class 223:228
Ident 229:236 text="LeafBox"
Extends 237:244
Ident 245:251 text="MidBox"
LBrace 252:253
Newline 253:254
Var 258:261
Ident 262:263 text="l"
Colon 263:264
Ident 265:268 text="int"
Newline 268:269
Newline 269:270
Constructor 274:285
LParen 285:286
Ident 286:287 text="l"
Colon 287:288
Ident 289:292 text="int"
RParen 292:293
LBrace 294:295
Newline 295:296
Super 304:309
LParen 309:310
Ident 310:311 text="l"
Plus 312:313
Number 314:315 text="1" suf="" isf=0 int=1
RParen 315:316
Newline 316:317
This 325:329
Dot 329:330
Ident 330:331 text="l"
Assign 332:333
Ident 334:335 text="l"
Star 336:337
Number 338:339 text="3" suf="" isf=0 int=3
Newline 339:340
RBrace 344:345
Newline 345:346
Newline 346:347
Fn 351:353
Ident 354:359 text="total"
LParen 359:360
RParen 360:361
Arrow 362:364
Ident 365:368 text="int"
LBrace 369:370
Newline 370:371
This 379:383
Dot 383:384
Ident 384:385 text="g"
Plus 386:387
This 388:392
Dot 392:393
Ident 393:394 text="m"
Plus 395:396
This 397:401
Dot 401:402
Ident 402:403 text="l"
Newline 403:404
RBrace 408:409
Newline 409:410
RBrace 410:411
Newline 411:412
Newline 412:413
Class 413:418
Ident 419:428 text="SynthBase"
LBrace 429:430
Newline 430:431
Var 435:438
Ident 439:441 text="s1"
Colon 441:442
Ident 443:446 text="int"
Newline 446:447
Var 451:454
Ident 455:457 text="s2"
Colon 457:458
Ident 459:462 text="int"
Newline 462:463
RBrace 463:464
Newline 464:465
Newline 465:466
Class 466:471
Ident 472:482 text="SynthChild"
Extends 483:490
Ident 491:500 text="SynthBase"
LBrace 501:502
Newline 502:503
Var 507:510
Ident 511:512 text="c"
Colon 512:513
Ident 514:517 text="int"
Newline 517:518
Newline 518:519
Constructor 523:534
LParen 534:535
Ident 535:536 text="c"
Colon 536:537
Ident 538:541 text="int"
RParen 541:542
LBrace 543:544
Newline 544:545
Super 553:558
LParen 558:559
Ident 559:560 text="c"
Comma 560:561
Ident 562:563 text="c"
Star 564:565
Number 566:568 text="10" suf="" isf=0 int=10
RParen 568:569
Newline 569:570
This 578:582
Dot 582:583
Ident 583:584 text="c"
Assign 585:586
Ident 587:588 text="c"
Newline 588:589
RBrace 593:594
Newline 594:595
RBrace 595:596
Newline 596:597
Newline 597:598
Class 598:603
Ident 604:611 text="StrBase"
LBrace 612:613
Newline 613:614
Var 618:621
Ident 622:626 text="name"
Colon 626:627
Ident 628:634 text="string"
Newline 634:635
Newline 635:636
Constructor 640:651
LParen 651:652
Ident 652:656 text="name"
Colon 656:657
Ident 658:664 text="string"
RParen 664:665
LBrace 666:667
Newline 667:668
This 676:680
Dot 680:681
Ident 681:685 text="name"
Assign 686:687
Ident 688:692 text="name"
Newline 692:693
RBrace 697:698
Newline 698:699
RBrace 699:700
Newline 700:701
Newline 701:702
Class 702:707
Ident 708:716 text="StrChild"
Extends 717:724
Ident 725:732 text="StrBase"
LBrace 733:734
Newline 734:735
Var 739:742
Ident 743:744 text="n"
Colon 744:745
Ident 746:749 text="int"
Newline 749:750
Var 754:757
Ident 758:763 text="extra"
Colon 763:764
Ident 765:771 text="string"
Assign 772:773
Str 774:777 T("!")
Newline 777:778
Newline 778:779
Constructor 783:794
LParen 794:795
Ident 795:799 text="name"
Colon 799:800
Ident 801:807 text="string"
Comma 807:808
Ident 809:810 text="n"
Colon 810:811
Ident 812:815 text="int"
RParen 815:816
LBrace 817:818
Newline 818:819
Super 827:832
LParen 832:833
Ident 833:837 text="name"
RParen 837:838
Newline 838:839
This 847:851
Dot 851:852
Ident 852:853 text="n"
Assign 854:855
Ident 856:857 text="n"
Newline 857:858
RBrace 862:863
Newline 863:864
RBrace 864:865
Newline 865:866
Newline 866:867
Class 867:872
Ident 873:881 text="UnitBase"
LBrace 882:883
Newline 883:884
Var 888:891
Ident 892:897 text="ready"
Colon 897:898
Ident 899:903 text="bool"
Assign 904:905
True 906:910
Newline 910:911
RBrace 911:912
Newline 912:913
Newline 913:914
Class 914:919
Ident 920:929 text="UnitChild"
Extends 930:937
Ident 938:946 text="UnitBase"
LBrace 947:948
Newline 948:949
Var 953:956
Ident 957:960 text="tag"
Colon 960:961
Ident 962:965 text="int"
Newline 965:966
Newline 966:967
Constructor 971:982
LParen 982:983
Ident 983:986 text="tag"
Colon 986:987
Ident 988:991 text="int"
RParen 991:992
LBrace 993:994
Newline 994:995
Super 1003:1008
LParen 1008:1009
RParen 1009:1010
Newline 1010:1011
This 1019:1023
Dot 1023:1024
Ident 1024:1027 text="tag"
Assign 1028:1029
Ident 1030:1033 text="tag"
Newline 1033:1034
RBrace 1038:1039
Newline 1039:1040
RBrace 1040:1041
Newline 1041:1042
Newline 1042:1043
Class 1043:1048
Ident 1049:1063 text="SynthBelowBase"
LBrace 1064:1065
Newline 1065:1066
Var 1070:1073
Ident 1074:1075 text="g"
Colon 1075:1076
Ident 1077:1080 text="int"
Assign 1081:1082
Number 1083:1086 text="100" suf="" isf=0 int=100
Newline 1086:1087
Newline 1087:1088
Constructor 1092:1103
LParen 1103:1104
Ident 1104:1105 text="g"
Colon 1105:1106
Ident 1107:1110 text="int"
RParen 1110:1111
LBrace 1112:1113
Newline 1113:1114
This 1122:1126
Dot 1126:1127
Ident 1127:1128 text="g"
Assign 1129:1130
This 1131:1135
Dot 1135:1136
Ident 1136:1137 text="g"
Plus 1138:1139
Ident 1140:1141 text="g"
Newline 1141:1142
RBrace 1146:1147
Newline 1147:1148
RBrace 1148:1149
Newline 1149:1150
Newline 1150:1151
Class 1151:1156
Ident 1157:1166 text="SynthLeaf"
Extends 1167:1174
Ident 1175:1189 text="SynthBelowBase"
LBrace 1190:1191
Newline 1191:1192
Var 1196:1199
Ident 1200:1201 text="l"
Colon 1201:1202
Ident 1203:1206 text="int"
Newline 1206:1207
RBrace 1207:1208
Newline 1208:1209
Newline 1209:1210
Class 1210:1215
Ident 1216:1224 text="AutoLeaf"
Extends 1225:1232
Ident 1233:1247 text="SynthBelowBase"
LBrace 1248:1249
Newline 1249:1250
Var 1254:1257
Ident 1258:1259 text="a"
Colon 1259:1260
Ident 1261:1264 text="int"
Assign 1265:1266
Number 1267:1268 text="5" suf="" isf=0 int=5
Newline 1268:1269
Var 1273:1276
Ident 1277:1278 text="b"
Colon 1278:1279
Ident 1280:1283 text="int"
Newline 1283:1284
RBrace 1284:1285
Newline 1285:1286
Newline 1286:1287
Class 1287:1292
Ident 1293:1300 text="DeepMid"
Extends 1301:1308
Ident 1309:1323 text="SynthBelowBase"
LBrace 1324:1325
Newline 1325:1326
Var 1330:1333
Ident 1334:1335 text="m"
Colon 1335:1336
Ident 1337:1340 text="int"
Assign 1341:1342
Number 1343:1344 text="1" suf="" isf=0 int=1
Newline 1344:1345
RBrace 1345:1346
Newline 1346:1347
Newline 1347:1348
Class 1348:1353
Ident 1354:1362 text="DeepLeaf"
Extends 1363:1370
Ident 1371:1378 text="DeepMid"
LBrace 1379:1380
Newline 1380:1381
Var 1385:1388
Ident 1389:1390 text="d"
Colon 1390:1391
Ident 1392:1395 text="int"
Newline 1395:1396
RBrace 1396:1397
Newline 1397:1398
Newline 1398:1399
Class 1399:1404
Ident 1405:1413 text="NamedBox"
LBrace 1414:1415
Newline 1415:1416
Var 1420:1423
Ident 1424:1425 text="x"
Colon 1425:1426
Ident 1427:1430 text="int"
Newline 1430:1431
Var 1435:1438
Ident 1439:1440 text="y"
Colon 1440:1441
Ident 1442:1445 text="int"
Newline 1445:1446
Newline 1446:1447
Constructor 1451:1462
LParen 1462:1463
Ident 1463:1464 text="x"
Colon 1464:1465
Ident 1466:1469 text="int"
Comma 1469:1470
Ident 1471:1472 text="y"
Colon 1472:1473
Ident 1474:1477 text="int"
RParen 1477:1478
LBrace 1479:1480
Newline 1480:1481
This 1489:1493
Dot 1493:1494
Ident 1494:1495 text="x"
Assign 1496:1497
Ident 1498:1499 text="x"
Newline 1499:1500
This 1508:1512
Dot 1512:1513
Ident 1513:1514 text="y"
Assign 1515:1516
Ident 1517:1518 text="y"
Newline 1518:1519
RBrace 1523:1524
Newline 1524:1525
Newline 1525:1526
Constructor 1530:1541
Dot 1541:1542
Ident 1542:1546 text="side"
LParen 1546:1547
Ident 1547:1548 text="n"
Colon 1548:1549
Ident 1550:1553 text="int"
RParen 1553:1554
LBrace 1555:1556
Newline 1556:1557
This 1565:1569
LParen 1569:1570
Ident 1570:1571 text="n"
Comma 1571:1572
Ident 1573:1574 text="n"
RParen 1574:1575
Newline 1575:1576
RBrace 1580:1581
Newline 1581:1582
RBrace 1582:1583
Newline 1583:1584
Newline 1584:1585
Class 1585:1590
Ident 1591:1600 text="NamedLeaf"
Extends 1601:1608
Ident 1609:1617 text="NamedBox"
LBrace 1618:1619
Newline 1619:1620
Var 1624:1627
Ident 1628:1629 text="z"
Colon 1629:1630
Ident 1631:1634 text="int"
Newline 1634:1635
Newline 1635:1636
Constructor 1640:1651
LParen 1651:1652
Ident 1652:1653 text="z"
Colon 1653:1654
Ident 1655:1658 text="int"
RParen 1658:1659
LBrace 1660:1661
Newline 1661:1662
Super 1670:1675
LParen 1675:1676
Ident 1676:1677 text="z"
Comma 1677:1678
Ident 1679:1680 text="z"
Star 1681:1682
Number 1683:1684 text="2" suf="" isf=0 int=2
RParen 1684:1685
Newline 1685:1686
This 1694:1698
Dot 1698:1699
Ident 1699:1700 text="z"
Assign 1701:1702
Ident 1703:1704 text="z"
Newline 1704:1705
RBrace 1709:1710
Newline 1710:1711
RBrace 1711:1712
Newline 1712:1713
Newline 1713:1714
Ident 1714:1722 text="describe"
LParen 1722:1723
Str 1723:1751 T("constructor super chaining")
Comma 1751:1752
LBrace 1753:1754
Newline 1754:1755
Ident 1759:1763 text="test"
LParen 1763:1764
Str 1764:1814 T("three-level explicit ctor chain inlines in order")
Comma 1814:1815
LBrace 1816:1817
Newline 1817:1818
Let 1826:1829
Ident 1830:1831 text="b"
Assign 1832:1833
Ident 1834:1841 text="LeafBox"
LParen 1841:1842
Number 1842:1843 text="5" suf="" isf=0 int=5
RParen 1843:1844
Newline 1844:1845
Ident 1853:1859 text="expect"
LParen 1859:1860
Ident 1860:1861 text="b"
Dot 1861:1862
Ident 1862:1863 text="g"
RParen 1863:1864
Dot 1864:1865
Ident 1865:1869 text="toBe"
LParen 1869:1870
Number 1870:1873 text="112" suf="" isf=0 int=112
RParen 1873:1874
Newline 1874:1875
Ident 1883:1889 text="expect"
LParen 1889:1890
Ident 1890:1891 text="b"
Dot 1891:1892
Ident 1892:1893 text="m"
RParen 1893:1894
Dot 1894:1895
Ident 1895:1899 text="toBe"
LParen 1899:1900
Number 1900:1901 text="6" suf="" isf=0 int=6
RParen 1901:1902
Newline 1902:1903
Ident 1911:1917 text="expect"
LParen 1917:1918
Ident 1918:1919 text="b"
Dot 1919:1920
Ident 1920:1921 text="l"
RParen 1921:1922
Dot 1922:1923
Ident 1923:1927 text="toBe"
LParen 1927:1928
Number 1928:1930 text="15" suf="" isf=0 int=15
RParen 1930:1931
Newline 1931:1932
Ident 1940:1946 text="expect"
LParen 1946:1947
Ident 1947:1948 text="b"
Dot 1948:1949
Ident 1949:1954 text="total"
LParen 1954:1955
RParen 1955:1956
RParen 1956:1957
Dot 1957:1958
Ident 1958:1962 text="toBe"
LParen 1962:1963
Number 1963:1966 text="133" suf="" isf=0 int=133
RParen 1966:1967
Newline 1967:1968
RBrace 1972:1973
RParen 1973:1974
Newline 1974:1975
Newline 1975:1976
Ident 1980:1984 text="test"
LParen 1984:1985
Str 1985:2048 T("explicit child ctor super-delegates into a synthesized parent")
Comma 2048:2049
LBrace 2050:2051
Newline 2051:2052
Let 2060:2063
Ident 2064:2065 text="s"
Assign 2066:2067
Ident 2068:2078 text="SynthChild"
LParen 2078:2079
Number 2079:2080 text="3" suf="" isf=0 int=3
RParen 2080:2081
Newline 2081:2082
Ident 2090:2096 text="expect"
LParen 2096:2097
Ident 2097:2098 text="s"
Dot 2098:2099
Ident 2099:2101 text="s1"
RParen 2101:2102
Dot 2102:2103
Ident 2103:2107 text="toBe"
LParen 2107:2108
Number 2108:2109 text="3" suf="" isf=0 int=3
RParen 2109:2110
Newline 2110:2111
Ident 2119:2125 text="expect"
LParen 2125:2126
Ident 2126:2127 text="s"
Dot 2127:2128
Ident 2128:2130 text="s2"
RParen 2130:2131
Dot 2131:2132
Ident 2132:2136 text="toBe"
LParen 2136:2137
Number 2137:2139 text="30" suf="" isf=0 int=30
RParen 2139:2140
Newline 2140:2141
Ident 2149:2155 text="expect"
LParen 2155:2156
Ident 2156:2157 text="s"
Dot 2157:2158
Ident 2158:2159 text="c"
RParen 2159:2160
Dot 2160:2161
Ident 2161:2165 text="toBe"
LParen 2165:2166
Number 2166:2167 text="3" suf="" isf=0 int=3
RParen 2167:2168
Newline 2168:2169
RBrace 2173:2174
RParen 2174:2175
Newline 2175:2176
Newline 2176:2177
Ident 2181:2185 text="test"
LParen 2185:2186
Str 2186:2236 T("pointer (string) parent param survives the chain")
Comma 2236:2237
LBrace 2238:2239
Newline 2239:2240
Let 2248:2251
Ident 2252:2254 text="sc"
Assign 2255:2256
Ident 2257:2265 text="StrChild"
LParen 2265:2266
Str 2266:2270 T("hi")
Comma 2270:2271
Number 2272:2273 text="9" suf="" isf=0 int=9
RParen 2273:2274
Newline 2274:2275
Ident 2283:2289 text="expect"
LParen 2289:2290
Ident 2290:2292 text="sc"
Dot 2292:2293
Ident 2293:2297 text="name"
RParen 2297:2298
Dot 2298:2299
Ident 2299:2303 text="toBe"
LParen 2303:2304
Str 2304:2308 T("hi")
RParen 2308:2309
Newline 2309:2310
Ident 2318:2324 text="expect"
LParen 2324:2325
Ident 2325:2327 text="sc"
Dot 2327:2328
Ident 2328:2329 text="n"
RParen 2329:2330
Dot 2330:2331
Ident 2331:2335 text="toBe"
LParen 2335:2336
Number 2336:2337 text="9" suf="" isf=0 int=9
RParen 2337:2338
Newline 2338:2339
Ident 2347:2353 text="expect"
LParen 2353:2354
Ident 2354:2356 text="sc"
Dot 2356:2357
Ident 2357:2362 text="extra"
RParen 2362:2363
Dot 2363:2364
Ident 2364:2368 text="toBe"
LParen 2368:2369
Str 2369:2372 T("!")
RParen 2372:2373
Newline 2373:2374
RBrace 2378:2379
RParen 2379:2380
Newline 2380:2381
Newline 2381:2382
Ident 2386:2390 text="test"
LParen 2390:2391
Str 2391:2435 T("zero-arg super() into a synthesized parent")
Comma 2435:2436
LBrace 2437:2438
Newline 2438:2439
Let 2447:2450
Ident 2451:2452 text="u"
Assign 2453:2454
Ident 2455:2464 text="UnitChild"
LParen 2464:2465
Number 2465:2466 text="4" suf="" isf=0 int=4
RParen 2466:2467
Newline 2467:2468
Ident 2476:2482 text="expect"
LParen 2482:2483
Ident 2483:2484 text="u"
Dot 2484:2485
Ident 2485:2490 text="ready"
RParen 2490:2491
Dot 2491:2492
Ident 2492:2496 text="toBe"
LParen 2496:2497
True 2497:2501
RParen 2501:2502
Newline 2502:2503
Ident 2511:2517 text="expect"
LParen 2517:2518
Ident 2518:2519 text="u"
Dot 2519:2520
Ident 2520:2523 text="tag"
RParen 2523:2524
Dot 2524:2525
Ident 2525:2529 text="toBe"
LParen 2529:2530
Number 2530:2531 text="4" suf="" isf=0 int=4
RParen 2531:2532
Newline 2532:2533
RBrace 2537:2538
RParen 2538:2539
Newline 2539:2540
Newline 2540:2541
Ident 2545:2549 text="test"
LParen 2549:2550
Str 2550:2619 T("synthesized ctor below an explicit-primary ancestor forwards params")
Comma 2619:2620
LBrace 2621:2622
Newline 2622:2623
Let 2631:2634
Ident 2635:2636 text="s"
Assign 2637:2638
Ident 2639:2648 text="SynthLeaf"
LParen 2648:2649
Number 2649:2650 text="5" suf="" isf=0 int=5
Comma 2650:2651
Number 2652:2653 text="7" suf="" isf=0 int=7
RParen 2653:2654
Newline 2654:2655
Ident 2663:2669 text="expect"
LParen 2669:2670
Ident 2670:2671 text="s"
Dot 2671:2672
Ident 2672:2673 text="g"
RParen 2673:2674
Dot 2674:2675
Ident 2675:2679 text="toBe"
LParen 2679:2680
Number 2680:2683 text="105" suf="" isf=0 int=105
RParen 2683:2684
Newline 2684:2685
Ident 2693:2699 text="expect"
LParen 2699:2700
Ident 2700:2701 text="s"
Dot 2701:2702
Ident 2702:2703 text="l"
RParen 2703:2704
Dot 2704:2705
Ident 2705:2709 text="toBe"
LParen 2709:2710
Number 2710:2711 text="7" suf="" isf=0 int=7
RParen 2711:2712
Newline 2712:2713
RBrace 2717:2718
RParen 2718:2719
Newline 2719:2720
Newline 2720:2721
Ident 2725:2729 text="test"
LParen 2729:2730
Str 2730:2805 T("synthesized ctor keeps sibling field initializers below explicit ancestor")
Comma 2805:2806
LBrace 2807:2808
Newline 2808:2809
Let 2817:2820
Ident 2821:2822 text="s"
Assign 2823:2824
Ident 2825:2833 text="AutoLeaf"
LParen 2833:2834
Number 2834:2835 text="2" suf="" isf=0 int=2
Comma 2835:2836
Number 2837:2838 text="9" suf="" isf=0 int=9
RParen 2838:2839
Newline 2839:2840
Ident 2848:2854 text="expect"
LParen 2854:2855
Ident 2855:2856 text="s"
Dot 2856:2857
Ident 2857:2858 text="g"
RParen 2858:2859
Dot 2859:2860
Ident 2860:2864 text="toBe"
LParen 2864:2865
Number 2865:2868 text="102" suf="" isf=0 int=102
RParen 2868:2869
Newline 2869:2870
Ident 2878:2884 text="expect"
LParen 2884:2885
Ident 2885:2886 text="s"
Dot 2886:2887
Ident 2887:2888 text="a"
RParen 2888:2889
Dot 2889:2890
Ident 2890:2894 text="toBe"
LParen 2894:2895
Number 2895:2896 text="5" suf="" isf=0 int=5
RParen 2896:2897
Newline 2897:2898
Ident 2906:2912 text="expect"
LParen 2912:2913
Ident 2913:2914 text="s"
Dot 2914:2915
Ident 2915:2916 text="b"
RParen 2916:2917
Dot 2917:2918
Ident 2918:2922 text="toBe"
LParen 2922:2923
Number 2923:2924 text="9" suf="" isf=0 int=9
RParen 2924:2925
Newline 2925:2926
RBrace 2930:2931
RParen 2931:2932
Newline 2932:2933
Newline 2933:2934
Ident 2938:2942 text="test"
LParen 2942:2943
Str 2943:3013 T("synthesized ctor chains down several levels to the explicit ancestor")
Comma 3013:3014
LBrace 3015:3016
Newline 3016:3017
Let 3025:3028
Ident 3029:3030 text="s"
Assign 3031:3032
Ident 3033:3041 text="DeepLeaf"
LParen 3041:3042
Number 3042:3043 text="3" suf="" isf=0 int=3
Comma 3043:3044
Number 3045:3046 text="4" suf="" isf=0 int=4
RParen 3046:3047
Newline 3047:3048
Ident 3056:3062 text="expect"
LParen 3062:3063
Ident 3063:3064 text="s"
Dot 3064:3065
Ident 3065:3066 text="g"
RParen 3066:3067
Dot 3067:3068
Ident 3068:3072 text="toBe"
LParen 3072:3073
Number 3073:3076 text="103" suf="" isf=0 int=103
RParen 3076:3077
Newline 3077:3078
Ident 3086:3092 text="expect"
LParen 3092:3093
Ident 3093:3094 text="s"
Dot 3094:3095
Ident 3095:3096 text="m"
RParen 3096:3097
Dot 3097:3098
Ident 3098:3102 text="toBe"
LParen 3102:3103
Number 3103:3104 text="1" suf="" isf=0 int=1
RParen 3104:3105
Newline 3105:3106
Ident 3114:3120 text="expect"
LParen 3120:3121
Ident 3121:3122 text="s"
Dot 3122:3123
Ident 3123:3124 text="d"
RParen 3124:3125
Dot 3125:3126
Ident 3126:3130 text="toBe"
LParen 3130:3131
Number 3131:3132 text="4" suf="" isf=0 int=4
RParen 3132:3133
Newline 3133:3134
RBrace 3138:3139
RParen 3139:3140
Newline 3140:3141
Newline 3141:3142
Ident 3146:3150 text="test"
LParen 3150:3151
Str 3151:3212 T("named ctor delegates through the primary and into the chain")
Comma 3212:3213
LBrace 3214:3215
Newline 3215:3216
Ident 3224:3230 text="expect"
LParen 3230:3231
Ident 3231:3239 text="NamedBox"
Dot 3239:3240
Ident 3240:3244 text="side"
LParen 3244:3245
Number 3245:3246 text="5" suf="" isf=0 int=5
RParen 3246:3247
Dot 3247:3248
Ident 3248:3249 text="x"
RParen 3249:3250
Dot 3250:3251
Ident 3251:3255 text="toBe"
LParen 3255:3256
Number 3256:3257 text="5" suf="" isf=0 int=5
RParen 3257:3258
Newline 3258:3259
Ident 3267:3273 text="expect"
LParen 3273:3274
Ident 3274:3282 text="NamedBox"
Dot 3282:3283
Ident 3283:3287 text="side"
LParen 3287:3288
Number 3288:3289 text="5" suf="" isf=0 int=5
RParen 3289:3290
Dot 3290:3291
Ident 3291:3292 text="y"
RParen 3292:3293
Dot 3293:3294
Ident 3294:3298 text="toBe"
LParen 3298:3299
Number 3299:3300 text="5" suf="" isf=0 int=5
RParen 3300:3301
Newline 3301:3302
Let 3310:3313
Ident 3314:3316 text="nc"
Assign 3317:3318
Ident 3319:3328 text="NamedLeaf"
LParen 3328:3329
Number 3329:3330 text="6" suf="" isf=0 int=6
RParen 3330:3331
Newline 3331:3332
Ident 3340:3346 text="expect"
LParen 3346:3347
Ident 3347:3349 text="nc"
Dot 3349:3350
Ident 3350:3351 text="x"
RParen 3351:3352
Dot 3352:3353
Ident 3353:3357 text="toBe"
LParen 3357:3358
Number 3358:3359 text="6" suf="" isf=0 int=6
RParen 3359:3360
Newline 3360:3361
Ident 3369:3375 text="expect"
LParen 3375:3376
Ident 3376:3378 text="nc"
Dot 3378:3379
Ident 3379:3380 text="y"
RParen 3380:3381
Dot 3381:3382
Ident 3382:3386 text="toBe"
LParen 3386:3387
Number 3387:3389 text="12" suf="" isf=0 int=12
RParen 3389:3390
Newline 3390:3391
Ident 3399:3405 text="expect"
LParen 3405:3406
Ident 3406:3408 text="nc"
Dot 3408:3409
Ident 3409:3410 text="z"
RParen 3410:3411
Dot 3411:3412
Ident 3412:3416 text="toBe"
LParen 3416:3417
Number 3417:3418 text="6" suf="" isf=0 int=6
RParen 3418:3419
Newline 3419:3420
RBrace 3424:3425
RParen 3425:3426
Newline 3426:3427
RBrace 3427:3428
RParen 3428:3429
Eof 3429:3429
