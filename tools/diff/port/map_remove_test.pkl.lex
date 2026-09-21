Ident 0:4 text="test"
Fn 5:7
Ident 8:30 text="map_remove_present_int"
LParen 30:31
RParen 31:32
LBrace 33:34
Newline 34:35
Let 39:42
Ident 43:44 text="m"
Assign 45:46
LBrace 47:48
Str 48:51 T("a")
Colon 51:52
Number 53:54 text="1" suf="" isf=0 int=1
Comma 54:55
Str 56:59 T("b")
Colon 59:60
Number 61:62 text="2" suf="" isf=0 int=2
RBrace 62:63
Newline 63:64
Let 68:71
Ident 72:79 text="removed"
Assign 80:81
Ident 82:83 text="m"
Dot 83:84
Ident 84:90 text="remove"
LParen 90:91
Str 91:94 T("a")
RParen 94:95
Newline 95:96
If 100:102
LParen 103:104
Let 104:107
Ident 108:112 text="some"
LParen 112:113
Ident 113:114 text="v"
RParen 114:115
Assign 116:117
Ident 118:125 text="removed"
RParen 125:126
LBrace 127:128
Newline 128:129
Ident 137:143 text="expect"
LParen 143:144
Ident 144:145 text="v"
RParen 145:146
Dot 146:147
Ident 147:151 text="toBe"
LParen 151:152
Number 152:153 text="1" suf="" isf=0 int=1
RParen 153:154
Newline 154:155
RBrace 159:160
Else 161:165
LBrace 166:167
Newline 167:168
Ident 176:182 text="expect"
LParen 182:183
Str 183:195 T("unexpected")
RParen 195:196
Dot 196:197
Ident 197:201 text="toBe"
LParen 201:202
Str 202:266 T("map_remove_present_int: remove returned none for a present key")
RParen 266:267
Newline 267:268
RBrace 272:273
Newline 273:274
Ident 278:284 text="expect"
LParen 284:285
Ident 285:286 text="m"
Dot 286:287
Ident 287:290 text="has"
LParen 290:291
Str 291:294 T("a")
RParen 294:295
RParen 295:296
Dot 296:297
Ident 297:301 text="toBe"
LParen 301:302
False 302:307
RParen 307:308
Newline 308:309
Ident 313:319 text="expect"
LParen 319:320
Ident 320:321 text="m"
Dot 321:322
Ident 322:325 text="has"
LParen 325:326
Str 326:329 T("b")
RParen 329:330
RParen 330:331
Dot 331:332
Ident 332:336 text="toBe"
LParen 336:337
True 337:341
RParen 341:342
Newline 342:343
Ident 347:353 text="expect"
LParen 353:354
Ident 354:357 text="len"
LParen 357:358
Ident 358:359 text="m"
Dot 359:360
Ident 360:366 text="values"
LParen 366:367
RParen 367:368
RParen 368:369
RParen 369:370
Dot 370:371
Ident 371:375 text="toBe"
LParen 375:376
Number 376:377 text="1" suf="" isf=0 int=1
RParen 377:378
Newline 378:379
RBrace 379:380
Newline 380:381
Newline 381:382
Ident 382:386 text="test"
Fn 387:389
Ident 390:415 text="map_remove_absent_is_none"
LParen 415:416
RParen 416:417
LBrace 418:419
Newline 419:420
Let 424:427
Ident 428:429 text="m"
Assign 430:431
LBrace 432:433
Str 433:436 T("a")
Colon 436:437
Number 438:439 text="7" suf="" isf=0 int=7
RBrace 439:440
Newline 440:441
Let 445:448
Ident 449:456 text="removed"
Assign 457:458
Ident 459:460 text="m"
Dot 460:461
Ident 461:467 text="remove"
LParen 467:468
Str 468:477 T("missing")
RParen 477:478
Newline 478:479
If 483:485
LParen 486:487
Let 487:490
Ident 491:495 text="some"
LParen 495:496
Ident 496:498 text="_v"
RParen 498:499
Assign 500:501
Ident 502:509 text="removed"
RParen 509:510
LBrace 511:512
Newline 512:513
Ident 521:527 text="expect"
LParen 527:528
Str 528:540 T("unexpected")
RParen 540:541
Dot 541:542
Ident 542:546 text="toBe"
LParen 546:547
Str 547:614 T("map_remove_absent_is_none: remove returned some for an absent key")
RParen 614:615
Newline 615:616
RBrace 620:621
Else 622:626
LBrace 627:628
Newline 628:629
Ident 637:643 text="expect"
LParen 643:644
True 644:648
RParen 648:649
Dot 649:650
Ident 650:654 text="toBe"
LParen 654:655
True 655:659
RParen 659:660
Newline 660:661
RBrace 665:666
Newline 666:667
Ident 671:677 text="expect"
LParen 677:678
Ident 678:679 text="m"
Dot 679:680
Ident 680:683 text="has"
LParen 683:684
Str 684:687 T("a")
RParen 687:688
RParen 688:689
Dot 689:690
Ident 690:694 text="toBe"
LParen 694:695
True 695:699
RParen 699:700
Newline 700:701
RBrace 701:702
Newline 702:703
Newline 703:704
Ident 704:708 text="test"
Fn 709:711
Ident 712:737 text="map_remove_none_coalecess"
LParen 737:738
RParen 738:739
LBrace 740:741
Newline 741:742
Let 746:749
Ident 750:751 text="m"
Assign 752:753
LBrace 754:755
Str 755:758 T("x")
Colon 758:759
Number 760:762 text="42" suf="" isf=0 int=42
RBrace 762:763
Newline 763:764
Let 768:771
Ident 772:773 text="v"
Assign 774:775
Ident 776:777 text="m"
Dot 777:778
Ident 778:784 text="remove"
LParen 784:785
Str 785:791 T("nope")
RParen 791:792
QuestionQuestion 793:795
Minus 796:797
Number 797:798 text="1" suf="" isf=0 int=1
Newline 798:799
Ident 803:809 text="expect"
LParen 809:810
Ident 810:811 text="v"
RParen 811:812
Dot 812:813
Ident 813:817 text="toBe"
LParen 817:818
Minus 818:819
Number 819:820 text="1" suf="" isf=0 int=1
RParen 820:821
Newline 821:822
Let 826:829
Ident 830:831 text="w"
Assign 832:833
Ident 834:835 text="m"
Dot 835:836
Ident 836:842 text="remove"
LParen 842:843
Str 843:846 T("x")
RParen 846:847
QuestionQuestion 848:850
Minus 851:852
Number 852:853 text="1" suf="" isf=0 int=1
Newline 853:854
Ident 858:864 text="expect"
LParen 864:865
Ident 865:866 text="w"
RParen 866:867
Dot 867:868
Ident 868:872 text="toBe"
LParen 872:873
Number 873:875 text="42" suf="" isf=0 int=42
RParen 875:876
Newline 876:877
RBrace 877:878
Newline 878:879
Newline 879:880
Ident 880:884 text="test"
Fn 885:887
Ident 888:911 text="map_remove_string_value"
LParen 911:912
RParen 912:913
LBrace 914:915
Newline 915:916
Let 920:923
Ident 924:925 text="m"
Assign 926:927
LBrace 928:929
Str 929:932 T("k")
Colon 932:933
Str 934:941 T("hello")
Comma 941:942
Str 943:946 T("j")
Colon 946:947
Str 948:955 T("world")
RBrace 955:956
Newline 956:957
Let 961:964
Ident 965:972 text="removed"
Assign 973:974
Ident 975:976 text="m"
Dot 976:977
Ident 977:983 text="remove"
LParen 983:984
Str 984:987 T("k")
RParen 987:988
Newline 988:989
If 993:995
LParen 996:997
Let 997:1000
Ident 1001:1005 text="some"
LParen 1005:1006
Ident 1006:1007 text="v"
RParen 1007:1008
Assign 1009:1010
Ident 1011:1018 text="removed"
RParen 1018:1019
LBrace 1020:1021
Newline 1021:1022
Ident 1030:1036 text="expect"
LParen 1036:1037
Ident 1037:1038 text="v"
RParen 1038:1039
Dot 1039:1040
Ident 1040:1044 text="toBe"
LParen 1044:1045
Str 1045:1052 T("hello")
RParen 1052:1053
Newline 1053:1054
RBrace 1058:1059
Else 1060:1064
LBrace 1065:1066
Newline 1066:1067
Ident 1075:1081 text="expect"
LParen 1081:1082
Str 1082:1094 T("unexpected")
RParen 1094:1095
Dot 1095:1096
Ident 1096:1100 text="toBe"
LParen 1100:1101
Str 1101:1166 T("map_remove_string_value: remove returned none for a present key")
RParen 1166:1167
Newline 1167:1168
RBrace 1172:1173
Newline 1173:1174
Ident 1178:1184 text="expect"
LParen 1184:1185
Ident 1185:1186 text="m"
Dot 1186:1187
Ident 1187:1190 text="has"
LParen 1190:1191
Str 1191:1194 T("k")
RParen 1194:1195
RParen 1195:1196
Dot 1196:1197
Ident 1197:1201 text="toBe"
LParen 1201:1202
False 1202:1207
RParen 1207:1208
Newline 1208:1209
RBrace 1209:1210
Newline 1210:1211
Newline 1211:1212
Ident 1212:1216 text="test"
Fn 1217:1219
Ident 1220:1254 text="map_remove_bool_and_double_removal"
LParen 1254:1255
RParen 1255:1256
LBrace 1257:1258
Newline 1258:1259
Let 1263:1266
Ident 1267:1268 text="m"
Assign 1269:1270
LBrace 1271:1272
Str 1272:1277 T("yes")
Colon 1277:1278
True 1279:1283
Comma 1283:1284
Str 1285:1288 T("n")
Colon 1288:1289
False 1290:1295
RBrace 1295:1296
Newline 1296:1297
Let 1301:1304
Ident 1305:1306 text="a"
Assign 1307:1308
Ident 1309:1310 text="m"
Dot 1310:1311
Ident 1311:1317 text="remove"
LParen 1317:1318
Str 1318:1323 T("yes")
RParen 1323:1324
Newline 1324:1325
If 1329:1331
LParen 1332:1333
Let 1333:1336
Ident 1337:1341 text="some"
LParen 1341:1342
Ident 1342:1343 text="v"
RParen 1343:1344
Assign 1345:1346
Ident 1347:1348 text="a"
RParen 1348:1349
LBrace 1350:1351
Newline 1351:1352
Ident 1360:1366 text="expect"
LParen 1366:1367
Ident 1367:1368 text="v"
RParen 1368:1369
Dot 1369:1370
Ident 1370:1374 text="toBe"
LParen 1374:1375
True 1375:1379
RParen 1379:1380
Newline 1380:1381
RBrace 1385:1386
Else 1387:1391
LBrace 1392:1393
Newline 1393:1394
Ident 1402:1408 text="expect"
LParen 1408:1409
Str 1409:1421 T("unexpected")
RParen 1421:1422
Dot 1422:1423
Ident 1423:1427 text="toBe"
LParen 1427:1428
Str 1428:1504 T("map_remove_bool_and_double_removal: remove returned none for a present key")
RParen 1504:1505
Newline 1505:1506
RBrace 1510:1511
Newline 1511:1512
Let 1516:1519
Ident 1520:1525 text="again"
Assign 1526:1527
Ident 1528:1529 text="m"
Dot 1529:1530
Ident 1530:1536 text="remove"
LParen 1536:1537
Str 1537:1542 T("yes")
RParen 1542:1543
Newline 1543:1544
If 1548:1550
LParen 1551:1552
Let 1552:1555
Ident 1556:1560 text="some"
LParen 1560:1561
Ident 1561:1563 text="_v"
RParen 1563:1564
Assign 1565:1566
Ident 1567:1572 text="again"
RParen 1572:1573
LBrace 1574:1575
Newline 1575:1576
Ident 1584:1590 text="expect"
LParen 1590:1591
Str 1591:1603 T("unexpected")
RParen 1603:1604
Dot 1604:1605
Ident 1605:1609 text="toBe"
LParen 1609:1610
Str 1610:1682 T("map_remove_bool_and_double_removal: a second remove should return none")
RParen 1682:1683
Newline 1683:1684
RBrace 1688:1689
Else 1690:1694
LBrace 1695:1696
Newline 1696:1697
Ident 1705:1711 text="expect"
LParen 1711:1712
True 1712:1716
RParen 1716:1717
Dot 1717:1718
Ident 1718:1722 text="toBe"
LParen 1722:1723
True 1723:1727
RParen 1727:1728
Newline 1728:1729
RBrace 1733:1734
Newline 1734:1735
RBrace 1735:1736
Newline 1736:1737
Newline 1737:1738
Ident 1738:1742 text="test"
Fn 1743:1745
Ident 1746:1768 text="map_remove_float_value"
LParen 1768:1769
RParen 1769:1770
LBrace 1771:1772
Newline 1772:1773
Let 1777:1780
Ident 1781:1782 text="m"
Assign 1783:1784
LBrace 1785:1786
Str 1786:1790 T("pi")
Colon 1790:1791
Number 1792:1796 text="3.14" suf="" isf=1 int=-
RBrace 1796:1797
Newline 1797:1798
Let 1802:1805
Ident 1806:1813 text="removed"
Assign 1814:1815
Ident 1816:1817 text="m"
Dot 1817:1818
Ident 1818:1824 text="remove"
LParen 1824:1825
Str 1825:1829 T("pi")
RParen 1829:1830
Newline 1830:1831
If 1835:1837
LParen 1838:1839
Let 1839:1842
Ident 1843:1847 text="some"
LParen 1847:1848
Ident 1848:1849 text="v"
RParen 1849:1850
Assign 1851:1852
Ident 1853:1860 text="removed"
RParen 1860:1861
LBrace 1862:1863
Newline 1863:1864
Ident 1872:1878 text="expect"
LParen 1878:1879
Ident 1879:1880 text="v"
RParen 1880:1881
Dot 1881:1882
Ident 1882:1886 text="toBe"
LParen 1886:1887
Number 1887:1891 text="3.14" suf="" isf=1 int=-
RParen 1891:1892
Newline 1892:1893
RBrace 1897:1898
Else 1899:1903
LBrace 1904:1905
Newline 1905:1906
Ident 1914:1920 text="expect"
LParen 1920:1921
Str 1921:1933 T("unexpected")
RParen 1933:1934
Dot 1934:1935
Ident 1935:1939 text="toBe"
LParen 1939:1940
Str 1940:2004 T("map_remove_float_value: remove returned none for a present key")
RParen 2004:2005
Newline 2005:2006
RBrace 2010:2011
Newline 2011:2012
RBrace 2012:2013
Eof 2013:2013
