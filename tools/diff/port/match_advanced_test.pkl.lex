Enum 0:4
Ident 5:15 text="Direction2"
LBrace 16:17
Newline 17:18
Ident 22:27 text="North"
Newline 27:28
Ident 32:37 text="South"
Newline 37:38
Ident 42:46 text="East"
Newline 46:47
Ident 51:55 text="West"
Newline 55:56
RBrace 56:57
Newline 57:58
Newline 58:59
Class 59:64
Ident 65:71 text="Point2"
LBrace 72:73
Newline 73:74
Var 78:81
Ident 82:83 text="x"
Colon 83:84
Ident 85:88 text="int"
Newline 88:89
Var 93:96
Ident 97:98 text="y"
Colon 98:99
Ident 100:103 text="int"
Newline 103:104
RBrace 104:105
Newline 105:106
Newline 106:107
Fn 107:109
Ident 110:117 text="sumPair"
LParen 117:118
Ident 118:119 text="p"
Colon 119:120
LParen 121:122
Ident 122:125 text="int"
Comma 125:126
Ident 127:130 text="int"
RParen 130:131
RParen 131:132
Arrow 133:135
Ident 136:139 text="int"
LBrace 140:141
Newline 141:142
Match 146:151
LParen 152:153
Ident 153:154 text="p"
RParen 154:155
LBrace 156:157
Newline 157:158
Case 166:170
LParen 171:172
Ident 172:173 text="a"
Comma 173:174
Ident 175:176 text="b"
RParen 176:177
Arrow 178:180
Ident 181:182 text="a"
Plus 183:184
Ident 185:186 text="b"
Newline 186:187
RBrace 191:192
Newline 192:193
RBrace 193:194
Newline 194:195
Newline 195:196
Fn 196:198
Ident 199:206 text="labelOf"
LParen 206:207
Ident 207:208 text="p"
Colon 208:209
LParen 210:211
Ident 211:214 text="int"
Comma 214:215
Ident 216:219 text="int"
RParen 219:220
RParen 220:221
Arrow 222:224
Ident 225:231 text="string"
LBrace 232:233
Newline 233:234
Match 238:243
LParen 244:245
Ident 245:246 text="p"
RParen 246:247
LBrace 248:249
Newline 249:250
Case 258:262
LParen 263:264
Number 264:265 text="0" suf="" isf=0 int=0
Comma 265:266
Number 267:268 text="0" suf="" isf=0 int=0
RParen 268:269
Arrow 270:272
Str 273:281 T("origin")
Newline 281:282
Case 290:294
LParen 295:296
Ident 296:297 text="a"
Comma 297:298
Ident 299:300 text="_"
RParen 300:301
If 302:304
Ident 305:306 text="a"
Gt 307:308
Number 309:310 text="0" suf="" isf=0 int=0
Arrow 311:313
Str 314:321 T("right")
Newline 321:322
Case 330:334
Ident 335:336 text="_"
Arrow 337:339
Str 340:347 T("other")
Newline 347:348
RBrace 352:353
Newline 353:354
RBrace 354:355
Newline 355:356
Newline 356:357
Fn 357:359
Ident 360:369 text="swapFirst"
LParen 369:370
Ident 370:371 text="q"
Colon 371:372
LParen 373:374
LParen 374:375
Ident 375:378 text="int"
Comma 378:379
Ident 380:383 text="int"
RParen 383:384
Comma 384:385
Ident 386:389 text="int"
RParen 389:390
RParen 390:391
Arrow 392:394
Ident 395:398 text="int"
LBrace 399:400
Newline 400:401
Match 405:410
LParen 411:412
Ident 412:413 text="q"
RParen 413:414
LBrace 415:416
Newline 416:417
Case 425:429
LParen 430:431
LParen 431:432
Ident 432:433 text="x"
Comma 433:434
Ident 435:436 text="y"
RParen 436:437
Comma 437:438
Ident 439:440 text="z"
RParen 440:441
Arrow 442:444
Ident 445:446 text="x"
Star 447:448
Ident 449:450 text="y"
Plus 451:452
Ident 453:454 text="z"
Newline 454:455
RBrace 459:460
Newline 460:461
RBrace 461:462
Newline 462:463
Newline 463:464
Fn 464:466
Ident 467:478 text="sumOfOption"
LParen 478:479
Ident 479:480 text="p"
Colon 480:481
LParen 482:483
Ident 483:486 text="int"
Comma 486:487
Ident 488:494 text="string"
RParen 494:495
Question 495:496
RParen 496:497
Arrow 498:500
Ident 501:507 text="string"
LBrace 508:509
Newline 509:510
Match 514:519
LParen 520:521
Ident 521:522 text="p"
RParen 522:523
LBrace 524:525
Newline 525:526
Case 534:538
Ident 539:543 text="some"
LParen 543:544
LParen 544:545
Ident 545:546 text="a"
Comma 546:547
Ident 548:549 text="b"
RParen 549:550
RParen 550:551
Arrow 552:554
Str 555:564 E[Ident 557:558 text="a"; Eof 559:559] T(":") E[Ident 561:562 text="b"; Eof 563:563]
Newline 564:565
Case 573:577
None 578:582
Arrow 583:585
Str 586:595 T("missing")
Newline 595:596
RBrace 600:601
Newline 601:602
RBrace 602:603
Newline 603:604
Newline 604:605
Fn 605:607
Ident 608:616 text="oneOrTwo"
LParen 616:617
Ident 617:618 text="n"
Colon 618:619
Ident 620:623 text="int"
RParen 623:624
Arrow 625:627
Ident 628:634 text="string"
LBrace 635:636
Newline 636:637
Match 641:646
LParen 647:648
Ident 648:649 text="n"
RParen 649:650
LBrace 651:652
Newline 652:653
Case 661:665
Number 666:667 text="1" suf="" isf=0 int=1
Pipe 668:669
Number 670:671 text="2" suf="" isf=0 int=2
Arrow 672:674
Str 675:682 T("small")
Newline 682:683
Case 691:695
Number 696:697 text="3" suf="" isf=0 int=3
Arrow 698:700
Str 701:708 T("three")
Newline 708:709
Case 717:721
Ident 722:723 text="_"
Arrow 724:726
Str 727:734 T("other")
Newline 734:735
RBrace 739:740
Newline 740:741
RBrace 741:742
Newline 742:743
Newline 743:744
Fn 744:746
Ident 747:754 text="dirName"
LParen 754:755
Ident 755:756 text="d"
Colon 756:757
Ident 758:768 text="Direction2"
RParen 768:769
Arrow 770:772
Ident 773:779 text="string"
LBrace 780:781
Newline 781:782
Match 786:791
LParen 792:793
Ident 793:794 text="d"
RParen 794:795
LBrace 796:797
Newline 797:798
Case 806:810
Ident 811:821 text="Direction2"
Dot 821:822
Ident 822:827 text="North"
Pipe 828:829
Ident 830:840 text="Direction2"
Dot 840:841
Ident 841:846 text="South"
Arrow 847:849
Str 850:860 T("vertical")
Newline 860:861
Case 869:873
Ident 874:884 text="Direction2"
Dot 884:885
Ident 885:889 text="East"
Pipe 890:891
Ident 892:902 text="Direction2"
Dot 902:903
Ident 903:907 text="West"
Arrow 908:910
Str 911:923 T("horizontal")
Newline 923:924
RBrace 928:929
Newline 929:930
RBrace 930:931
Newline 931:932
Newline 932:933
Fn 933:935
Ident 936:946 text="noDupFirst"
LParen 946:947
Ident 947:948 text="m"
Colon 948:949
Ident 950:953 text="int"
Question 953:954
RParen 954:955
Arrow 956:958
Ident 959:965 text="string"
LBrace 966:967
Newline 967:968
Match 972:977
LParen 978:979
Ident 979:980 text="m"
RParen 980:981
LBrace 982:983
Newline 983:984
Case 992:996
Ident 997:1001 text="some"
LParen 1001:1002
Number 1002:1003 text="1" suf="" isf=0 int=1
RParen 1003:1004
Pipe 1005:1006
Ident 1007:1011 text="some"
LParen 1011:1012
Number 1012:1013 text="2" suf="" isf=0 int=2
RParen 1013:1014
Arrow 1015:1017
Str 1018:1025 T("small")
Newline 1025:1026
Case 1034:1038
Ident 1039:1043 text="some"
LParen 1043:1044
Ident 1044:1045 text="v"
RParen 1045:1046
Arrow 1047:1049
Str 1050:1059 T("big:") E[Ident 1056:1057 text="v"; Eof 1058:1058]
Newline 1059:1060
Case 1068:1072
None 1073:1077
Arrow 1078:1080
Str 1081:1087 T("none")
Newline 1087:1088
RBrace 1092:1093
Newline 1093:1094
RBrace 1094:1095
Newline 1095:1096
Newline 1096:1097
Fn 1097:1099
Ident 1100:1107 text="ifLetOr"
LParen 1107:1108
Ident 1108:1109 text="n"
Colon 1109:1110
Ident 1111:1114 text="int"
RParen 1114:1115
Arrow 1116:1118
Ident 1119:1122 text="int"
LBrace 1123:1124
Newline 1124:1125
If 1129:1131
LParen 1132:1133
Let 1133:1136
Number 1137:1138 text="1" suf="" isf=0 int=1
Pipe 1139:1140
Number 1141:1142 text="2" suf="" isf=0 int=2
Assign 1143:1144
Ident 1145:1146 text="n"
RParen 1146:1147
LBrace 1148:1149
Newline 1149:1150
Number 1158:1160 text="10" suf="" isf=0 int=10
Newline 1160:1161
RBrace 1165:1166
Else 1167:1171
LBrace 1172:1173
Newline 1173:1174
Ident 1182:1183 text="n"
Newline 1183:1184
RBrace 1188:1189
Newline 1189:1190
RBrace 1190:1191
Newline 1191:1192
Newline 1192:1193
Fn 1193:1195
Ident 1196:1206 text="ifLetTuple"
LParen 1206:1207
Ident 1207:1208 text="p"
Colon 1208:1209
LParen 1210:1211
Ident 1211:1214 text="int"
Comma 1214:1215
Ident 1216:1219 text="int"
RParen 1219:1220
RParen 1220:1221
Arrow 1222:1224
Ident 1225:1228 text="int"
LBrace 1229:1230
Newline 1230:1231
If 1235:1237
LParen 1238:1239
Let 1239:1242
LParen 1243:1244
Ident 1244:1245 text="a"
Comma 1245:1246
Ident 1247:1248 text="b"
RParen 1248:1249
Assign 1250:1251
Ident 1252:1253 text="p"
RParen 1253:1254
LBrace 1255:1256
Newline 1256:1257
Ident 1265:1266 text="a"
Star 1267:1268
Ident 1269:1270 text="b"
Newline 1270:1271
RBrace 1275:1276
Else 1277:1281
LBrace 1282:1283
Newline 1283:1284
Number 1292:1293 text="0" suf="" isf=0 int=0
Newline 1293:1294
RBrace 1298:1299
Newline 1299:1300
RBrace 1300:1301
Newline 1301:1302
Newline 1302:1303
Fn 1303:1305
Ident 1306:1318 text="listIdentity"
LParen 1318:1319
Ident 1319:1320 text="l"
Colon 1320:1321
Ident 1322:1326 text="List"
Lt 1326:1327
Ident 1327:1330 text="int"
Gt 1330:1331
RParen 1331:1332
Arrow 1333:1335
Ident 1336:1339 text="int"
LBrace 1340:1341
Newline 1341:1342
Match 1346:1351
LParen 1352:1353
Ident 1353:1354 text="l"
RParen 1354:1355
LBrace 1356:1357
Newline 1357:1358
Case 1366:1370
Ident 1371:1373 text="xs"
Arrow 1374:1376
Ident 1377:1380 text="len"
LParen 1380:1381
Ident 1381:1383 text="xs"
RParen 1383:1384
Newline 1384:1385
RBrace 1389:1390
Newline 1390:1391
RBrace 1391:1392
Newline 1392:1393
Newline 1393:1394
Fn 1394:1396
Ident 1397:1410 text="classIdentity"
LParen 1410:1411
Ident 1411:1412 text="p"
Colon 1412:1413
Ident 1414:1420 text="Point2"
RParen 1420:1421
Arrow 1422:1424
Ident 1425:1428 text="int"
LBrace 1429:1430
Newline 1430:1431
Match 1435:1440
LParen 1441:1442
Ident 1442:1443 text="p"
RParen 1443:1444
LBrace 1445:1446
Newline 1446:1447
Case 1455:1459
Ident 1460:1461 text="o"
Arrow 1462:1464
Ident 1465:1466 text="o"
Dot 1466:1467
Ident 1467:1468 text="x"
Plus 1469:1470
Ident 1471:1472 text="o"
Dot 1472:1473
Ident 1473:1474 text="y"
Newline 1474:1475
RBrace 1479:1480
Newline 1480:1481
RBrace 1481:1482
Newline 1482:1483
Newline 1483:1484
Fn 1484:1486
Ident 1487:1498 text="mapIdentity"
LParen 1498:1499
Ident 1499:1500 text="m"
Colon 1500:1501
Ident 1502:1505 text="Map"
Lt 1505:1506
Ident 1506:1512 text="string"
Comma 1512:1513
Ident 1514:1517 text="int"
Gt 1517:1518
RParen 1518:1519
Arrow 1520:1522
Ident 1523:1526 text="int"
LBrace 1527:1528
Newline 1528:1529
Match 1533:1538
LParen 1539:1540
Ident 1540:1541 text="m"
RParen 1541:1542
LBrace 1543:1544
Newline 1544:1545
Case 1553:1557
Ident 1558:1560 text="dm"
Arrow 1561:1563
Ident 1564:1567 text="len"
LParen 1567:1568
Ident 1568:1570 text="dm"
RParen 1570:1571
Newline 1571:1572
RBrace 1576:1577
Newline 1577:1578
RBrace 1578:1579
Newline 1579:1580
Newline 1580:1581
Fn 1581:1583
Ident 1584:1598 text="ifLetListIsPtr"
LParen 1598:1599
Ident 1599:1600 text="l"
Colon 1600:1601
Ident 1602:1606 text="List"
Lt 1606:1607
Ident 1607:1610 text="int"
Gt 1610:1611
RParen 1611:1612
Arrow 1613:1615
Ident 1616:1619 text="int"
LBrace 1620:1621
Newline 1621:1622
If 1626:1628
LParen 1629:1630
Let 1630:1633
Ident 1634:1635 text="x"
Assign 1636:1637
Ident 1638:1639 text="l"
RParen 1639:1640
LBrace 1641:1642
Newline 1642:1643
Ident 1651:1654 text="len"
LParen 1654:1655
Ident 1655:1656 text="x"
RParen 1656:1657
Newline 1657:1658
RBrace 1662:1663
Else 1664:1668
LBrace 1669:1670
Newline 1670:1671
Number 1679:1680 text="0" suf="" isf=0 int=0
Newline 1680:1681
RBrace 1685:1686
Newline 1686:1687
RBrace 1687:1688
Newline 1688:1689
Newline 1689:1690
Fn 1690:1692
Ident 1693:1712 text="firstOfListOfTuples"
LParen 1712:1713
Ident 1713:1717 text="rows"
Colon 1717:1718
Ident 1719:1723 text="List"
Lt 1723:1724
LParen 1724:1725
Ident 1725:1728 text="int"
Comma 1728:1729
Ident 1730:1736 text="string"
RParen 1736:1737
Gt 1737:1738
RParen 1738:1739
Arrow 1740:1742
Ident 1743:1749 text="string"
LBrace 1750:1751
Newline 1751:1752
Match 1756:1761
LParen 1762:1763
Ident 1763:1767 text="rows"
LBracket 1767:1768
Number 1768:1769 text="0" suf="" isf=0 int=0
RBracket 1769:1770
RParen 1770:1771
LBrace 1772:1773
Newline 1773:1774
Case 1782:1786
LParen 1787:1788
Ident 1788:1789 text="_"
Comma 1789:1790
Ident 1791:1795 text="name"
RParen 1795:1796
Arrow 1797:1799
Ident 1800:1804 text="name"
Newline 1804:1805
RBrace 1809:1810
Newline 1810:1811
RBrace 1811:1812
Newline 1812:1813
Newline 1813:1814
Ident 1814:1818 text="test"
Fn 1819:1821
Ident 1822:1847 text="tuple_values_and_bindings"
LParen 1847:1848
RParen 1848:1849
LBrace 1850:1851
Newline 1851:1852
Ident 1856:1862 text="expect"
LParen 1862:1863
Ident 1863:1870 text="sumPair"
LParen 1870:1871
LParen 1871:1872
Number 1872:1873 text="3" suf="" isf=0 int=3
Comma 1873:1874
Number 1875:1876 text="4" suf="" isf=0 int=4
RParen 1876:1877
RParen 1877:1878
RParen 1878:1879
Dot 1879:1880
Ident 1880:1884 text="toBe"
LParen 1884:1885
Number 1885:1886 text="7" suf="" isf=0 int=7
RParen 1886:1887
Newline 1887:1888
Ident 1892:1898 text="expect"
LParen 1898:1899
Ident 1899:1906 text="sumPair"
LParen 1906:1907
LParen 1907:1908
Number 1908:1909 text="0" suf="" isf=0 int=0
Comma 1909:1910
Number 1911:1912 text="0" suf="" isf=0 int=0
RParen 1912:1913
RParen 1913:1914
RParen 1914:1915
Dot 1915:1916
Ident 1916:1920 text="toBe"
LParen 1920:1921
Number 1921:1922 text="0" suf="" isf=0 int=0
RParen 1922:1923
Newline 1923:1924
RBrace 1924:1925
Newline 1925:1926
Newline 1926:1927
Ident 1927:1931 text="test"
Fn 1932:1934
Ident 1935:1963 text="tuple_literal_case_and_guard"
LParen 1963:1964
RParen 1964:1965
LBrace 1966:1967
Newline 1967:1968
Ident 1972:1978 text="expect"
LParen 1978:1979
Ident 1979:1986 text="labelOf"
LParen 1986:1987
LParen 1987:1988
Number 1988:1989 text="0" suf="" isf=0 int=0
Comma 1989:1990
Number 1991:1992 text="0" suf="" isf=0 int=0
RParen 1992:1993
RParen 1993:1994
RParen 1994:1995
Dot 1995:1996
Ident 1996:2000 text="toBe"
LParen 2000:2001
Str 2001:2009 T("origin")
RParen 2009:2010
Newline 2010:2011
Ident 2015:2021 text="expect"
LParen 2021:2022
Ident 2022:2029 text="labelOf"
LParen 2029:2030
LParen 2030:2031
Number 2031:2032 text="2" suf="" isf=0 int=2
Comma 2032:2033
Number 2034:2035 text="9" suf="" isf=0 int=9
RParen 2035:2036
RParen 2036:2037
RParen 2037:2038
Dot 2038:2039
Ident 2039:2043 text="toBe"
LParen 2043:2044
Str 2044:2051 T("right")
RParen 2051:2052
Newline 2052:2053
Ident 2057:2063 text="expect"
LParen 2063:2064
Ident 2064:2071 text="labelOf"
LParen 2071:2072
LParen 2072:2073
Number 2073:2074 text="0" suf="" isf=0 int=0
Comma 2074:2075
Number 2076:2077 text="5" suf="" isf=0 int=5
RParen 2077:2078
RParen 2078:2079
RParen 2079:2080
Dot 2080:2081
Ident 2081:2085 text="toBe"
LParen 2085:2086
Str 2086:2093 T("other")
RParen 2093:2094
Newline 2094:2095
Ident 2099:2105 text="expect"
LParen 2105:2106
Ident 2106:2113 text="labelOf"
LParen 2113:2114
LParen 2114:2115
Minus 2115:2116
Number 2116:2117 text="1" suf="" isf=0 int=1
Comma 2117:2118
Number 2119:2120 text="1" suf="" isf=0 int=1
RParen 2120:2121
RParen 2121:2122
RParen 2122:2123
Dot 2123:2124
Ident 2124:2128 text="toBe"
LParen 2128:2129
Str 2129:2136 T("other")
RParen 2136:2137
Newline 2137:2138
RBrace 2138:2139
Newline 2139:2140
Newline 2140:2141
Ident 2141:2145 text="test"
Fn 2146:2148
Ident 2149:2170 text="nested_tuple_patterns"
LParen 2170:2171
RParen 2171:2172
LBrace 2173:2174
Newline 2174:2175
Ident 2179:2185 text="expect"
LParen 2185:2186
Ident 2186:2195 text="swapFirst"
LParen 2195:2196
LParen 2196:2197
LParen 2197:2198
Number 2198:2199 text="2" suf="" isf=0 int=2
Comma 2199:2200
Number 2201:2202 text="3" suf="" isf=0 int=3
RParen 2202:2203
Comma 2203:2204
Number 2205:2206 text="4" suf="" isf=0 int=4
RParen 2206:2207
RParen 2207:2208
RParen 2208:2209
Dot 2209:2210
Ident 2210:2214 text="toBe"
LParen 2214:2215
Number 2215:2217 text="10" suf="" isf=0 int=10
RParen 2217:2218
Newline 2218:2219
RBrace 2219:2220
Newline 2220:2221
Newline 2221:2222
Ident 2222:2226 text="test"
Fn 2227:2229
Ident 2230:2253 text="some_with_tuple_payload"
LParen 2253:2254
RParen 2254:2255
LBrace 2256:2257
Newline 2257:2258
Let 2262:2265
Ident 2266:2268 text="sp"
Assign 2269:2270
LParen 2271:2272
Number 2272:2273 text="7" suf="" isf=0 int=7
Comma 2273:2274
Str 2275:2278 T("x")
RParen 2278:2279
As 2280:2282
Question 2282:2283
LParen 2284:2285
Ident 2285:2288 text="int"
Comma 2288:2289
Ident 2290:2296 text="string"
RParen 2296:2297
Newline 2297:2298
Ident 2302:2308 text="expect"
LParen 2308:2309
Ident 2309:2320 text="sumOfOption"
LParen 2320:2321
Ident 2321:2323 text="sp"
RParen 2323:2324
RParen 2324:2325
Dot 2325:2326
Ident 2326:2330 text="toBe"
LParen 2330:2331
Str 2331:2336 T("7:x")
RParen 2336:2337
Newline 2337:2338
Let 2342:2345
Ident 2346:2348 text="np"
Colon 2348:2349
LParen 2350:2351
Ident 2351:2354 text="int"
Comma 2354:2355
Ident 2356:2362 text="string"
RParen 2362:2363
Question 2363:2364
Assign 2365:2366
None 2367:2371
Newline 2371:2372
Ident 2376:2382 text="expect"
LParen 2382:2383
Ident 2383:2394 text="sumOfOption"
LParen 2394:2395
Ident 2395:2397 text="np"
RParen 2397:2398
RParen 2398:2399
Dot 2399:2400
Ident 2400:2404 text="toBe"
LParen 2404:2405
Str 2405:2414 T("missing")
RParen 2414:2415
Newline 2415:2416
RBrace 2416:2417
Newline 2417:2418
Newline 2418:2419
Ident 2419:2423 text="test"
Fn 2424:2426
Ident 2427:2446 text="or_pattern_literals"
LParen 2446:2447
RParen 2447:2448
LBrace 2449:2450
Newline 2450:2451
Ident 2455:2461 text="expect"
LParen 2461:2462
Ident 2462:2470 text="oneOrTwo"
LParen 2470:2471
Number 2471:2472 text="1" suf="" isf=0 int=1
RParen 2472:2473
RParen 2473:2474
Dot 2474:2475
Ident 2475:2479 text="toBe"
LParen 2479:2480
Str 2480:2487 T("small")
RParen 2487:2488
Newline 2488:2489
Ident 2493:2499 text="expect"
LParen 2499:2500
Ident 2500:2508 text="oneOrTwo"
LParen 2508:2509
Number 2509:2510 text="2" suf="" isf=0 int=2
RParen 2510:2511
RParen 2511:2512
Dot 2512:2513
Ident 2513:2517 text="toBe"
LParen 2517:2518
Str 2518:2525 T("small")
RParen 2525:2526
Newline 2526:2527
Ident 2531:2537 text="expect"
LParen 2537:2538
Ident 2538:2546 text="oneOrTwo"
LParen 2546:2547
Number 2547:2548 text="3" suf="" isf=0 int=3
RParen 2548:2549
RParen 2549:2550
Dot 2550:2551
Ident 2551:2555 text="toBe"
LParen 2555:2556
Str 2556:2563 T("three")
RParen 2563:2564
Newline 2564:2565
Ident 2569:2575 text="expect"
LParen 2575:2576
Ident 2576:2584 text="oneOrTwo"
LParen 2584:2585
Number 2585:2586 text="9" suf="" isf=0 int=9
RParen 2586:2587
RParen 2587:2588
Dot 2588:2589
Ident 2589:2593 text="toBe"
LParen 2593:2594
Str 2594:2601 T("other")
RParen 2601:2602
Newline 2602:2603
RBrace 2603:2604
Newline 2604:2605
Newline 2605:2606
Ident 2606:2610 text="test"
Fn 2611:2613
Ident 2614:2638 text="or_pattern_enum_variants"
LParen 2638:2639
RParen 2639:2640
LBrace 2641:2642
Newline 2642:2643
Ident 2647:2653 text="expect"
LParen 2653:2654
Ident 2654:2661 text="dirName"
LParen 2661:2662
Ident 2662:2672 text="Direction2"
Dot 2672:2673
Ident 2673:2678 text="North"
RParen 2678:2679
RParen 2679:2680
Dot 2680:2681
Ident 2681:2685 text="toBe"
LParen 2685:2686
Str 2686:2696 T("vertical")
RParen 2696:2697
Newline 2697:2698
Ident 2702:2708 text="expect"
LParen 2708:2709
Ident 2709:2716 text="dirName"
LParen 2716:2717
Ident 2717:2727 text="Direction2"
Dot 2727:2728
Ident 2728:2733 text="South"
RParen 2733:2734
RParen 2734:2735
Dot 2735:2736
Ident 2736:2740 text="toBe"
LParen 2740:2741
Str 2741:2751 T("vertical")
RParen 2751:2752
Newline 2752:2753
Ident 2757:2763 text="expect"
LParen 2763:2764
Ident 2764:2771 text="dirName"
LParen 2771:2772
Ident 2772:2782 text="Direction2"
Dot 2782:2783
Ident 2783:2787 text="East"
RParen 2787:2788
RParen 2788:2789
Dot 2789:2790
Ident 2790:2794 text="toBe"
LParen 2794:2795
Str 2795:2807 T("horizontal")
RParen 2807:2808
Newline 2808:2809
Ident 2813:2819 text="expect"
LParen 2819:2820
Ident 2820:2827 text="dirName"
LParen 2827:2828
Ident 2828:2838 text="Direction2"
Dot 2838:2839
Ident 2839:2843 text="West"
RParen 2843:2844
RParen 2844:2845
Dot 2845:2846
Ident 2846:2850 text="toBe"
LParen 2850:2851
Str 2851:2863 T("horizontal")
RParen 2863:2864
Newline 2864:2865
RBrace 2865:2866
Newline 2866:2867
Newline 2867:2868
Ident 2868:2872 text="test"
Fn 2873:2875
Ident 2876:2914 text="or_pattern_options_with_nested_literal"
LParen 2914:2915
RParen 2915:2916
LBrace 2917:2918
Newline 2918:2919
Let 2923:2926
Ident 2927:2928 text="m"
Colon 2928:2929
Ident 2930:2933 text="int"
Question 2933:2934
Assign 2935:2936
Number 2937:2938 text="1" suf="" isf=0 int=1
As 2939:2941
Question 2941:2942
Ident 2943:2946 text="int"
Newline 2946:2947
Ident 2951:2957 text="expect"
LParen 2957:2958
Ident 2958:2968 text="noDupFirst"
LParen 2968:2969
Ident 2969:2970 text="m"
RParen 2970:2971
RParen 2971:2972
Dot 2972:2973
Ident 2973:2977 text="toBe"
LParen 2977:2978
Str 2978:2985 T("small")
RParen 2985:2986
Newline 2986:2987
Let 2991:2994
Ident 2995:2996 text="n"
Colon 2996:2997
Ident 2998:3001 text="int"
Question 3001:3002
Assign 3003:3004
Number 3005:3006 text="2" suf="" isf=0 int=2
As 3007:3009
Question 3009:3010
Ident 3011:3014 text="int"
Newline 3014:3015
Ident 3019:3025 text="expect"
LParen 3025:3026
Ident 3026:3036 text="noDupFirst"
LParen 3036:3037
Ident 3037:3038 text="n"
RParen 3038:3039
RParen 3039:3040
Dot 3040:3041
Ident 3041:3045 text="toBe"
LParen 3045:3046
Str 3046:3053 T("small")
RParen 3053:3054
Newline 3054:3055
Let 3059:3062
Ident 3063:3064 text="b"
Colon 3064:3065
Ident 3066:3069 text="int"
Question 3069:3070
Assign 3071:3072
Number 3073:3074 text="7" suf="" isf=0 int=7
As 3075:3077
Question 3077:3078
Ident 3079:3082 text="int"
Newline 3082:3083
Ident 3087:3093 text="expect"
LParen 3093:3094
Ident 3094:3104 text="noDupFirst"
LParen 3104:3105
Ident 3105:3106 text="b"
RParen 3106:3107
RParen 3107:3108
Dot 3108:3109
Ident 3109:3113 text="toBe"
LParen 3113:3114
Str 3114:3121 T("big:7")
RParen 3121:3122
Newline 3122:3123
Let 3127:3130
Ident 3131:3132 text="u"
Colon 3132:3133
Ident 3134:3137 text="int"
Question 3137:3138
Assign 3139:3140
None 3141:3145
Newline 3145:3146
Ident 3150:3156 text="expect"
LParen 3156:3157
Ident 3157:3167 text="noDupFirst"
LParen 3167:3168
Ident 3168:3169 text="u"
RParen 3169:3170
RParen 3170:3171
Dot 3171:3172
Ident 3172:3176 text="toBe"
LParen 3176:3177
Str 3177:3183 T("none")
RParen 3183:3184
Newline 3184:3185
RBrace 3185:3186
Newline 3186:3187
Newline 3187:3188
Ident 3188:3192 text="test"
Fn 3193:3195
Ident 3196:3216 text="or_pattern_in_if_let"
LParen 3216:3217
RParen 3217:3218
LBrace 3219:3220
Newline 3220:3221
Ident 3225:3231 text="expect"
LParen 3231:3232
Ident 3232:3239 text="ifLetOr"
LParen 3239:3240
Number 3240:3241 text="1" suf="" isf=0 int=1
RParen 3241:3242
RParen 3242:3243
Dot 3243:3244
Ident 3244:3248 text="toBe"
LParen 3248:3249
Number 3249:3251 text="10" suf="" isf=0 int=10
RParen 3251:3252
Newline 3252:3253
Ident 3257:3263 text="expect"
LParen 3263:3264
Ident 3264:3271 text="ifLetOr"
LParen 3271:3272
Number 3272:3273 text="2" suf="" isf=0 int=2
RParen 3273:3274
RParen 3274:3275
Dot 3275:3276
Ident 3276:3280 text="toBe"
LParen 3280:3281
Number 3281:3283 text="10" suf="" isf=0 int=10
RParen 3283:3284
Newline 3284:3285
Ident 3289:3295 text="expect"
LParen 3295:3296
Ident 3296:3303 text="ifLetOr"
LParen 3303:3304
Number 3304:3305 text="5" suf="" isf=0 int=5
RParen 3305:3306
RParen 3306:3307
Dot 3307:3308
Ident 3308:3312 text="toBe"
LParen 3312:3313
Number 3313:3314 text="5" suf="" isf=0 int=5
RParen 3314:3315
Newline 3315:3316
RBrace 3316:3317
Newline 3317:3318
Newline 3318:3319
Ident 3319:3323 text="test"
Fn 3324:3326
Ident 3327:3354 text="if_let_tuple_binds_elements"
LParen 3354:3355
RParen 3355:3356
LBrace 3357:3358
Newline 3358:3359
Ident 3363:3369 text="expect"
LParen 3369:3370
Ident 3370:3380 text="ifLetTuple"
LParen 3380:3381
LParen 3381:3382
Number 3382:3383 text="3" suf="" isf=0 int=3
Comma 3383:3384
Number 3385:3386 text="4" suf="" isf=0 int=4
RParen 3386:3387
RParen 3387:3388
RParen 3388:3389
Dot 3389:3390
Ident 3390:3394 text="toBe"
LParen 3394:3395
Number 3395:3397 text="12" suf="" isf=0 int=12
RParen 3397:3398
Newline 3398:3399
RBrace 3399:3400
Newline 3400:3401
Newline 3401:3402
Ident 3402:3406 text="test"
Fn 3407:3409
Ident 3410:3435 text="composite_scrutinees_bind"
LParen 3435:3436
RParen 3436:3437
LBrace 3438:3439
Newline 3439:3440
Let 3444:3447
Ident 3448:3449 text="l"
Assign 3450:3451
LBracket 3452:3453
Number 3453:3454 text="1" suf="" isf=0 int=1
Comma 3454:3455
Number 3456:3457 text="2" suf="" isf=0 int=2
Comma 3457:3458
Number 3459:3460 text="3" suf="" isf=0 int=3
RBracket 3460:3461
Newline 3461:3462
Ident 3466:3472 text="expect"
LParen 3472:3473
Ident 3473:3485 text="listIdentity"
LParen 3485:3486
Ident 3486:3487 text="l"
RParen 3487:3488
RParen 3488:3489
Dot 3489:3490
Ident 3490:3494 text="toBe"
LParen 3494:3495
Number 3495:3496 text="3" suf="" isf=0 int=3
RParen 3496:3497
Newline 3497:3498
Let 3502:3505
Ident 3506:3507 text="o"
Assign 3508:3509
Ident 3510:3516 text="Point2"
LParen 3516:3517
Number 3517:3518 text="0" suf="" isf=0 int=0
Comma 3518:3519
Number 3520:3521 text="0" suf="" isf=0 int=0
RParen 3521:3522
Newline 3522:3523
Ident 3527:3533 text="expect"
LParen 3533:3534
Ident 3534:3547 text="classIdentity"
LParen 3547:3548
Ident 3548:3549 text="o"
RParen 3549:3550
RParen 3550:3551
Dot 3551:3552
Ident 3552:3556 text="toBe"
LParen 3556:3557
Number 3557:3558 text="0" suf="" isf=0 int=0
RParen 3558:3559
Newline 3559:3560
Let 3564:3567
Ident 3568:3569 text="m"
Assign 3570:3571
LBrace 3572:3573
Str 3574:3577 T("a")
Colon 3577:3578
Number 3579:3580 text="1" suf="" isf=0 int=1
RBrace 3581:3582
Newline 3582:3583
Ident 3587:3593 text="expect"
LParen 3593:3594
Ident 3594:3605 text="mapIdentity"
LParen 3605:3606
Ident 3606:3607 text="m"
RParen 3607:3608
RParen 3608:3609
Dot 3609:3610
Ident 3610:3614 text="toBe"
LParen 3614:3615
Number 3615:3616 text="1" suf="" isf=0 int=1
RParen 3616:3617
Newline 3617:3618
RBrace 3618:3619
Newline 3619:3620
Newline 3620:3621
Ident 3621:3625 text="test"
Fn 3626:3628
Ident 3629:3654 text="if_let_list_uses_ptr_slot"
LParen 3654:3655
RParen 3655:3656
LBrace 3657:3658
Newline 3658:3659
Let 3663:3666
Ident 3667:3668 text="l"
Assign 3669:3670
LBracket 3671:3672
Number 3672:3673 text="5" suf="" isf=0 int=5
Comma 3673:3674
Number 3675:3676 text="6" suf="" isf=0 int=6
RBracket 3676:3677
Newline 3677:3678
Ident 3682:3688 text="expect"
LParen 3688:3689
Ident 3689:3703 text="ifLetListIsPtr"
LParen 3703:3704
Ident 3704:3705 text="l"
RParen 3705:3706
RParen 3706:3707
Dot 3707:3708
Ident 3708:3712 text="toBe"
LParen 3712:3713
Number 3713:3714 text="2" suf="" isf=0 int=2
RParen 3714:3715
Newline 3715:3716
RBrace 3716:3717
Newline 3717:3718
Newline 3718:3719
Ident 3719:3723 text="test"
Fn 3724:3726
Ident 3727:3746 text="tuples_inside_lists"
LParen 3746:3747
RParen 3747:3748
LBrace 3749:3750
Newline 3750:3751
Let 3755:3758
Ident 3759:3763 text="rows"
Colon 3763:3764
Ident 3765:3769 text="List"
Lt 3769:3770
LParen 3770:3771
Ident 3771:3774 text="int"
Comma 3774:3775
Ident 3776:3782 text="string"
RParen 3782:3783
Gt 3783:3784
Assign 3785:3786
LBracket 3787:3788
LParen 3788:3789
Number 3789:3790 text="1" suf="" isf=0 int=1
Comma 3790:3791
Str 3792:3797 T("one")
RParen 3797:3798
Comma 3798:3799
LParen 3800:3801
Number 3801:3802 text="2" suf="" isf=0 int=2
Comma 3802:3803
Str 3804:3809 T("two")
RParen 3809:3810
RBracket 3810:3811
Newline 3811:3812
Ident 3816:3822 text="expect"
LParen 3822:3823
Ident 3823:3842 text="firstOfListOfTuples"
LParen 3842:3843
Ident 3843:3847 text="rows"
RParen 3847:3848
RParen 3848:3849
Dot 3849:3850
Ident 3850:3854 text="toBe"
LParen 3854:3855
Str 3855:3860 T("one")
RParen 3860:3861
Newline 3861:3862
RBrace 3862:3863
Eof 3863:3863
