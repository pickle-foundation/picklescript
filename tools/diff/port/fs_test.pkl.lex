Ident 0:4 text="test"
Fn 5:7
Ident 8:31 text="fs_write_read_roundtrip"
LParen 31:32
RParen 32:33
LBrace 34:35
Newline 35:36
Let 40:43
Ident 44:48 text="path"
Assign 49:50
Str 51:84 T("tests/pickle/__fs_roundtrip.tmp")
Newline 84:85
Let 89:92
Ident 93:95 text="ok"
Assign 96:97
Ident 98:108 text="write_file"
LParen 108:109
Ident 109:113 text="path"
Comma 113:114
Str 115:138 T("hello fs\nsecond line")
RParen 138:139
Newline 139:140
Ident 144:150 text="expect"
LParen 150:151
Ident 151:153 text="ok"
RParen 153:154
Dot 154:155
Ident 155:159 text="toBe"
LParen 159:160
True 160:164
RParen 164:165
Newline 165:166
Let 170:173
Ident 174:178 text="data"
Assign 179:180
Ident 181:190 text="read_file"
LParen 190:191
Ident 191:195 text="path"
RParen 195:196
Newline 196:197
If 201:203
LParen 204:205
Let 205:208
Ident 209:213 text="some"
LParen 213:214
Ident 214:217 text="txt"
RParen 217:218
Assign 219:220
Ident 221:225 text="data"
RParen 225:226
LBrace 227:228
Newline 228:229
Ident 237:243 text="expect"
LParen 243:244
Ident 244:247 text="txt"
RParen 247:248
Dot 248:249
Ident 249:253 text="toBe"
LParen 253:254
Str 254:277 T("hello fs\nsecond line")
RParen 277:278
Newline 278:279
RBrace 283:284
Else 285:289
LBrace 290:291
Newline 291:292
Ident 300:306 text="expect"
LParen 306:307
Str 307:319 T("unexpected")
RParen 319:320
Dot 320:321
Ident 321:325 text="toBe"
LParen 325:326
Str 326:387 T("roundtrip: read_file returned none after a successful write")
RParen 387:388
Newline 388:389
RBrace 393:394
Newline 394:395
RBrace 395:396
Newline 396:397
Newline 397:398
Ident 398:402 text="test"
Fn 403:405
Ident 406:429 text="fs_utf8_bytes_roundtrip"
LParen 429:430
RParen 430:431
LBrace 432:433
Newline 433:434
Newline 515:516
Let 520:523
Ident 524:528 text="path"
Assign 529:530
Str 531:559 T("tests/pickle/__fs_utf8.tmp")
Newline 559:560
Let 564:567
Ident 568:570 text="ok"
Assign 571:572
Ident 573:583 text="write_file"
LParen 583:584
Ident 584:588 text="path"
Comma 588:589
Str 590:616 T("héllo → done")
RParen 616:617
Newline 617:618
Ident 622:628 text="expect"
LParen 628:629
Ident 629:631 text="ok"
RParen 631:632
Dot 632:633
Ident 633:637 text="toBe"
LParen 637:638
True 638:642
RParen 642:643
Newline 643:644
Let 648:651
Ident 652:656 text="data"
Assign 657:658
Ident 659:668 text="read_file"
LParen 668:669
Ident 669:673 text="path"
RParen 673:674
Newline 674:675
If 679:681
LParen 682:683
Let 683:686
Ident 687:691 text="some"
LParen 691:692
Ident 692:695 text="txt"
RParen 695:696
Assign 697:698
Ident 699:703 text="data"
RParen 703:704
LBrace 705:706
Newline 706:707
Ident 715:721 text="expect"
LParen 721:722
Ident 722:725 text="txt"
RParen 725:726
Dot 726:727
Ident 727:731 text="toBe"
LParen 731:732
Str 732:758 T("héllo → done")
RParen 758:759
Newline 759:760
RBrace 764:765
Else 766:770
LBrace 771:772
Newline 772:773
Ident 781:787 text="expect"
LParen 787:788
Str 788:800 T("unexpected")
RParen 800:801
Dot 801:802
Ident 802:806 text="toBe"
LParen 806:807
Str 807:863 T("utf8: read_file returned none after a successful write")
RParen 863:864
Newline 864:865
RBrace 869:870
Newline 870:871
RBrace 871:872
Newline 872:873
Newline 873:874
Ident 874:878 text="test"
Fn 879:881
Ident 882:905 text="fs_read_missing_is_none"
LParen 905:906
RParen 906:907
LBrace 908:909
Newline 909:910
Let 914:917
Ident 918:922 text="data"
Assign 923:924
Ident 925:934 text="read_file"
LParen 934:935
Str 935:973 T("tests/pickle/__fs_does_not_exist.tmp")
RParen 973:974
Newline 974:975
If 979:981
LParen 982:983
Let 983:986
Ident 987:991 text="some"
LParen 991:992
Ident 992:994 text="_t"
RParen 994:995
Assign 996:997
Ident 998:1002 text="data"
RParen 1002:1003
LBrace 1004:1005
Newline 1005:1006
Ident 1014:1020 text="expect"
LParen 1020:1021
Str 1021:1033 T("unexpected")
RParen 1033:1034
Dot 1034:1035
Ident 1035:1039 text="toBe"
LParen 1039:1040
Str 1040:1073 T("missing file read returned some")
RParen 1073:1074
Newline 1074:1075
RBrace 1079:1080
Else 1081:1085
LBrace 1086:1087
Newline 1087:1088
Ident 1096:1102 text="expect"
LParen 1102:1103
True 1103:1107
RParen 1107:1108
Dot 1108:1109
Ident 1109:1113 text="toBe"
LParen 1113:1114
True 1114:1118
RParen 1118:1119
Newline 1119:1120
RBrace 1124:1125
Newline 1125:1126
RBrace 1126:1127
Newline 1127:1128
Newline 1128:1129
Ident 1129:1133 text="test"
Fn 1134:1136
Ident 1137:1151 text="fs_file_exists"
LParen 1151:1152
RParen 1152:1153
LBrace 1154:1155
Newline 1155:1156
Ident 1160:1166 text="expect"
LParen 1166:1167
Ident 1167:1178 text="file_exists"
LParen 1178:1179
Str 1179:1205 T("tests/pickle/fs_test.pkl")
RParen 1205:1206
RParen 1206:1207
Dot 1207:1208
Ident 1208:1212 text="toBe"
LParen 1212:1213
True 1213:1217
RParen 1217:1218
Newline 1218:1219
Ident 1223:1229 text="expect"
LParen 1229:1230
Ident 1230:1241 text="file_exists"
LParen 1241:1242
Str 1242:1282 T("tests/pickle/__fs_does_not_exist_2.tmp")
RParen 1282:1283
RParen 1283:1284
Dot 1284:1285
Ident 1285:1289 text="toBe"
LParen 1289:1290
False 1290:1295
RParen 1295:1296
Newline 1296:1297
Ident 1301:1307 text="expect"
LParen 1307:1308
Ident 1308:1319 text="file_exists"
LParen 1319:1320
Str 1320:1334 T("tests/pickle")
RParen 1334:1335
RParen 1335:1336
Dot 1336:1337
Ident 1337:1341 text="toBe"
LParen 1341:1342
True 1342:1346
RParen 1346:1347
Newline 1347:1348
RBrace 1348:1349
Newline 1349:1350
Newline 1350:1351
Ident 1351:1355 text="test"
Fn 1356:1358
Ident 1359:1380 text="fs_overwrite_existing"
LParen 1380:1381
RParen 1381:1382
LBrace 1383:1384
Newline 1384:1385
Let 1389:1392
Ident 1393:1397 text="path"
Assign 1398:1399
Str 1400:1433 T("tests/pickle/__fs_overwrite.tmp")
Newline 1433:1434
Ident 1438:1444 text="expect"
LParen 1444:1445
Ident 1445:1455 text="write_file"
LParen 1455:1456
Ident 1456:1460 text="path"
Comma 1460:1461
Str 1462:1469 T("first")
RParen 1469:1470
RParen 1470:1471
Dot 1471:1472
Ident 1472:1476 text="toBe"
LParen 1476:1477
True 1477:1481
RParen 1481:1482
Newline 1482:1483
Ident 1487:1493 text="expect"
LParen 1493:1494
Ident 1494:1504 text="write_file"
LParen 1504:1505
Ident 1505:1509 text="path"
Comma 1509:1510
Str 1511:1519 T("second")
RParen 1519:1520
RParen 1520:1521
Dot 1521:1522
Ident 1522:1526 text="toBe"
LParen 1526:1527
True 1527:1531
RParen 1531:1532
Newline 1532:1533
Let 1537:1540
Ident 1541:1545 text="data"
Assign 1546:1547
Ident 1548:1557 text="read_file"
LParen 1557:1558
Ident 1558:1562 text="path"
RParen 1562:1563
Newline 1563:1564
If 1568:1570
LParen 1571:1572
Let 1572:1575
Ident 1576:1580 text="some"
LParen 1580:1581
Ident 1581:1584 text="txt"
RParen 1584:1585
Assign 1586:1587
Ident 1588:1592 text="data"
RParen 1592:1593
LBrace 1594:1595
Newline 1595:1596
Ident 1604:1610 text="expect"
LParen 1610:1611
Ident 1611:1614 text="txt"
RParen 1614:1615
Dot 1615:1616
Ident 1616:1620 text="toBe"
LParen 1620:1621
Str 1621:1629 T("second")
RParen 1629:1630
Newline 1630:1631
RBrace 1635:1636
Else 1637:1641
LBrace 1642:1643
Newline 1643:1644
Ident 1652:1658 text="expect"
LParen 1658:1659
Str 1659:1671 T("unexpected")
RParen 1671:1672
Dot 1672:1673
Ident 1673:1677 text="toBe"
LParen 1677:1678
Str 1678:1739 T("overwrite: read_file returned none after a successful write")
RParen 1739:1740
Newline 1740:1741
RBrace 1745:1746
Newline 1746:1747
RBrace 1747:1748
Newline 1748:1749
Newline 1749:1750
Ident 1750:1754 text="test"
Fn 1755:1757
Ident 1758:1778 text="fs_match_over_option"
LParen 1778:1779
RParen 1779:1780
LBrace 1781:1782
Newline 1782:1783
Let 1787:1790
Ident 1791:1795 text="data"
Assign 1796:1797
Ident 1798:1807 text="read_file"
LParen 1807:1808
Str 1808:1834 T("tests/pickle/fs_test.pkl")
RParen 1834:1835
Newline 1835:1836
Match 1840:1845
LParen 1846:1847
Ident 1847:1851 text="data"
RParen 1851:1852
LBrace 1853:1854
Newline 1854:1855
Case 1863:1867
Ident 1868:1872 text="some"
LParen 1872:1873
Ident 1873:1875 text="_t"
RParen 1875:1876
Arrow 1877:1879
Ident 1880:1886 text="expect"
LParen 1886:1887
True 1887:1891
RParen 1891:1892
Dot 1892:1893
Ident 1893:1897 text="toBe"
LParen 1897:1898
True 1898:1902
RParen 1902:1903
Newline 1903:1904
Case 1912:1916
None 1917:1921
Arrow 1922:1924
Ident 1925:1931 text="expect"
LParen 1931:1932
Str 1932:1944 T("unexpected")
RParen 1944:1945
Dot 1945:1946
Ident 1946:1950 text="toBe"
LParen 1950:1951
Str 1951:1986 T("match: existing file read as none")
RParen 1986:1987
Newline 1987:1988
RBrace 1992:1993
Newline 1993:1994
RBrace 1994:1995
Newline 1995:1996
Newline 1996:1997
Ident 1997:2001 text="test"
Fn 2002:2004
Ident 2005:2034 text="fs_delete_removes_and_reports"
LParen 2034:2035
RParen 2035:2036
LBrace 2037:2038
Newline 2038:2039
Let 2043:2046
Ident 2047:2051 text="path"
Assign 2052:2053
Str 2054:2081 T("tests/pickle/__fs_del.tmp")
Newline 2081:2082
Ident 2086:2092 text="expect"
LParen 2092:2093
Ident 2093:2103 text="write_file"
LParen 2103:2104
Ident 2104:2108 text="path"
Comma 2108:2109
Str 2110:2113 T("x")
RParen 2113:2114
RParen 2114:2115
Dot 2115:2116
Ident 2116:2120 text="toBe"
LParen 2120:2121
True 2121:2125
RParen 2125:2126
Newline 2126:2127
Ident 2131:2137 text="expect"
LParen 2137:2138
Ident 2138:2144 text="delete"
LParen 2144:2145
Ident 2145:2149 text="path"
RParen 2149:2150
RParen 2150:2151
Dot 2151:2152
Ident 2152:2156 text="toBe"
LParen 2156:2157
True 2157:2161
RParen 2161:2162
Newline 2162:2163
Ident 2167:2173 text="expect"
LParen 2173:2174
Ident 2174:2185 text="file_exists"
LParen 2185:2186
Ident 2186:2190 text="path"
RParen 2190:2191
RParen 2191:2192
Dot 2192:2193
Ident 2193:2197 text="toBe"
LParen 2197:2198
False 2198:2203
RParen 2203:2204
Newline 2204:2205
Ident 2209:2215 text="expect"
LParen 2215:2216
Ident 2216:2222 text="delete"
LParen 2222:2223
Ident 2223:2227 text="path"
RParen 2227:2228
RParen 2228:2229
Dot 2229:2230
Ident 2230:2234 text="toBe"
LParen 2234:2235
False 2235:2240
RParen 2240:2241
Newline 2241:2242
Ident 2246:2252 text="expect"
LParen 2252:2253
Ident 2253:2259 text="delete"
LParen 2259:2260
Str 2260:2297 T("tests/pickle/__fs_never_existed.tmp")
RParen 2297:2298
RParen 2298:2299
Dot 2299:2300
Ident 2300:2304 text="toBe"
LParen 2304:2305
False 2305:2310
RParen 2310:2311
Newline 2311:2312
RBrace 2312:2313
Newline 2313:2314
Newline 2314:2315
Ident 2315:2319 text="test"
Fn 2320:2322
Ident 2323:2352 text="fs_list_dir_and_delete_in_dir"
LParen 2352:2353
RParen 2353:2354
LBrace 2355:2356
Newline 2356:2357
Let 2361:2364
Ident 2365:2368 text="dir"
Assign 2369:2370
Str 2371:2393 T("tests/pickle/__fsdir")
Newline 2393:2394
Newline 2468:2469
Newline 2542:2543
If 2547:2549
LParen 2550:2551
Ident 2551:2562 text="file_exists"
LParen 2562:2563
Ident 2563:2566 text="dir"
RParen 2566:2567
RParen 2567:2568
LBrace 2569:2570
Newline 2570:2571
If 2579:2581
LParen 2582:2583
Let 2583:2586
Ident 2587:2591 text="some"
LParen 2591:2592
Ident 2592:2596 text="prev"
RParen 2596:2597
Assign 2598:2599
Ident 2600:2608 text="list_dir"
LParen 2608:2609
Ident 2609:2612 text="dir"
RParen 2612:2613
RParen 2613:2614
LBrace 2615:2616
Newline 2616:2617
For 2629:2632
LParen 2633:2634
Ident 2634:2635 text="p"
In 2636:2638
Ident 2639:2643 text="prev"
RParen 2643:2644
LBrace 2645:2646
Newline 2646:2647
Ident 2663:2669 text="delete"
LParen 2669:2670
Ident 2670:2671 text="p"
RParen 2671:2672
Newline 2672:2673
RBrace 2685:2686
Newline 2686:2687
RBrace 2695:2696
Newline 2696:2697
Ident 2705:2711 text="delete"
LParen 2711:2712
Ident 2712:2715 text="dir"
RParen 2715:2716
Newline 2716:2717
RBrace 2721:2722
Newline 2722:2723
Ident 2727:2733 text="expect"
LParen 2733:2734
Ident 2734:2739 text="mkdir"
LParen 2739:2740
Ident 2740:2743 text="dir"
RParen 2743:2744
RParen 2744:2745
Dot 2745:2746
Ident 2746:2750 text="toBe"
LParen 2750:2751
True 2751:2755
RParen 2755:2756
Newline 2756:2757
Ident 2761:2771 text="write_file"
LParen 2771:2772
Str 2772:2787 E[Ident 2774:2777 text="dir"; Eof 2778:2778] T("/one.txt")
Comma 2787:2788
Str 2789:2792 T("1")
RParen 2792:2793
Newline 2793:2794
Ident 2798:2808 text="write_file"
LParen 2808:2809
Str 2809:2824 E[Ident 2811:2814 text="dir"; Eof 2815:2815] T("/two.txt")
Comma 2824:2825
Str 2826:2829 T("2")
RParen 2829:2830
Newline 2830:2831
Let 2835:2838
Ident 2839:2846 text="entries"
Assign 2847:2848
Ident 2849:2857 text="list_dir"
LParen 2857:2858
Ident 2858:2861 text="dir"
RParen 2861:2862
Newline 2862:2863
Var 2867:2870
Ident 2871:2872 text="n"
Assign 2873:2874
Number 2875:2876 text="0" suf="" isf=0 int=0
Newline 2876:2877
Var 2881:2884
Ident 2885:2892 text="saw_one"
Assign 2893:2894
False 2895:2900
Newline 2900:2901
Var 2905:2908
Ident 2909:2916 text="saw_two"
Assign 2917:2918
False 2919:2924
Newline 2924:2925
If 2929:2931
LParen 2932:2933
Let 2933:2936
Ident 2937:2941 text="some"
LParen 2941:2942
Ident 2942:2944 text="es"
RParen 2944:2945
Assign 2946:2947
Ident 2948:2955 text="entries"
RParen 2955:2956
LBrace 2957:2958
Newline 2958:2959
For 2967:2970
LParen 2971:2972
Ident 2972:2973 text="p"
In 2974:2976
Ident 2977:2979 text="es"
RParen 2979:2980
LBrace 2981:2982
Newline 2982:2983
Ident 2995:2996 text="n"
PlusEq 2997:2999
Number 3000:3001 text="1" suf="" isf=0 int=1
Newline 3001:3002
If 3014:3016
LParen 3017:3018
Ident 3018:3031 text="path_contains"
LParen 3031:3032
Ident 3032:3033 text="p"
Comma 3033:3034
Str 3035:3044 T("one.txt")
RParen 3044:3045
RParen 3045:3046
LBrace 3047:3048
Newline 3048:3049
Ident 3065:3072 text="saw_one"
Assign 3073:3074
True 3075:3079
Newline 3079:3080
RBrace 3092:3093
Newline 3093:3094
If 3106:3108
LParen 3109:3110
Ident 3110:3123 text="path_contains"
LParen 3123:3124
Ident 3124:3125 text="p"
Comma 3125:3126
Str 3127:3136 T("two.txt")
RParen 3136:3137
RParen 3137:3138
LBrace 3139:3140
Newline 3140:3141
Ident 3157:3164 text="saw_two"
Assign 3165:3166
True 3167:3171
Newline 3171:3172
RBrace 3184:3185
Newline 3185:3186
RBrace 3194:3195
Newline 3195:3196
RBrace 3200:3201
Newline 3201:3202
Ident 3206:3212 text="expect"
LParen 3212:3213
Ident 3213:3214 text="n"
RParen 3214:3215
Dot 3215:3216
Ident 3216:3220 text="toBe"
LParen 3220:3221
Number 3221:3222 text="2" suf="" isf=0 int=2
RParen 3222:3223
Newline 3223:3224
Ident 3228:3234 text="expect"
LParen 3234:3235
Ident 3235:3242 text="saw_one"
RParen 3242:3243
Dot 3243:3244
Ident 3244:3248 text="toBe"
LParen 3248:3249
True 3249:3253
RParen 3253:3254
Newline 3254:3255
Ident 3259:3265 text="expect"
LParen 3265:3266
Ident 3266:3273 text="saw_two"
RParen 3273:3274
Dot 3274:3275
Ident 3275:3279 text="toBe"
LParen 3279:3280
True 3280:3284
RParen 3284:3285
Newline 3285:3286
Ident 3290:3296 text="expect"
LParen 3296:3297
Ident 3297:3303 text="delete"
LParen 3303:3304
Str 3304:3319 E[Ident 3306:3309 text="dir"; Eof 3310:3310] T("/one.txt")
RParen 3319:3320
RParen 3320:3321
Dot 3321:3322
Ident 3322:3326 text="toBe"
LParen 3326:3327
True 3327:3331
RParen 3331:3332
Newline 3332:3333
Ident 3337:3343 text="expect"
LParen 3343:3344
Ident 3344:3350 text="delete"
LParen 3350:3351
Str 3351:3366 E[Ident 3353:3356 text="dir"; Eof 3357:3357] T("/two.txt")
RParen 3366:3367
RParen 3367:3368
Dot 3368:3369
Ident 3369:3373 text="toBe"
LParen 3373:3374
True 3374:3378
RParen 3378:3379
Newline 3379:3380
Ident 3384:3390 text="expect"
LParen 3390:3391
Ident 3391:3397 text="delete"
LParen 3397:3398
Ident 3398:3401 text="dir"
RParen 3401:3402
RParen 3402:3403
Dot 3403:3404
Ident 3404:3408 text="toBe"
LParen 3408:3409
True 3409:3413
RParen 3413:3414
Newline 3414:3415
RBrace 3415:3416
Newline 3416:3417
Newline 3417:3418
Fn 3418:3420
Ident 3421:3434 text="path_contains"
LParen 3434:3435
Ident 3435:3436 text="s"
Colon 3436:3437
Ident 3438:3444 text="string"
Comma 3444:3445
Ident 3446:3449 text="sub"
Colon 3449:3450
Ident 3451:3457 text="string"
RParen 3457:3458
Colon 3458:3459
Ident 3460:3464 text="bool"
LBrace 3465:3466
Newline 3466:3467
Let 3471:3474
Ident 3475:3477 text="sb"
Assign 3478:3479
Ident 3480:3485 text="bytes"
LParen 3485:3486
Ident 3486:3489 text="sub"
RParen 3489:3490
Newline 3490:3491
Let 3495:3498
Ident 3499:3502 text="sbl"
Assign 3503:3504
Ident 3505:3508 text="len"
LParen 3508:3509
Ident 3509:3511 text="sb"
RParen 3511:3512
Newline 3512:3513
If 3517:3519
LParen 3520:3521
Ident 3521:3524 text="sbl"
EqEq 3525:3527
Number 3528:3529 text="0" suf="" isf=0 int=0
RParen 3529:3530
LBrace 3531:3532
Newline 3532:3533
Return 3541:3547
True 3548:3552
Newline 3552:3553
RBrace 3557:3558
Newline 3558:3559
Let 3563:3566
Ident 3567:3568 text="b"
Assign 3569:3570
Ident 3571:3576 text="bytes"
LParen 3576:3577
Ident 3577:3578 text="s"
RParen 3578:3579
Newline 3579:3580
Var 3584:3587
Ident 3588:3589 text="i"
Assign 3590:3591
Number 3592:3593 text="0" suf="" isf=0 int=0
Newline 3593:3594
While 3598:3603
LParen 3604:3605
Ident 3605:3606 text="i"
Plus 3607:3608
Ident 3609:3612 text="sbl"
Le 3613:3615
Ident 3616:3619 text="len"
LParen 3619:3620
Ident 3620:3621 text="b"
RParen 3621:3622
RParen 3622:3623
LBrace 3624:3625
Newline 3625:3626
Var 3634:3637
Ident 3638:3639 text="j"
Assign 3640:3641
Number 3642:3643 text="0" suf="" isf=0 int=0
Newline 3643:3644
Var 3652:3655
Ident 3656:3658 text="ok"
Assign 3659:3660
True 3661:3665
Newline 3665:3666
While 3674:3679
LParen 3680:3681
Ident 3681:3682 text="j"
Lt 3683:3684
Ident 3685:3688 text="sbl"
RParen 3688:3689
LBrace 3690:3691
Newline 3691:3692
If 3704:3706
LParen 3707:3708
Ident 3708:3709 text="b"
LBracket 3709:3710
Ident 3710:3711 text="i"
Plus 3712:3713
Ident 3714:3715 text="j"
RBracket 3715:3716
NotEq 3717:3719
Ident 3720:3722 text="sb"
LBracket 3722:3723
Ident 3723:3724 text="j"
RBracket 3724:3725
RParen 3725:3726
LBrace 3727:3728
Newline 3728:3729
Ident 3745:3747 text="ok"
Assign 3748:3749
False 3750:3755
Newline 3755:3756
RBrace 3768:3769
Newline 3769:3770
Ident 3782:3783 text="j"
PlusEq 3784:3786
Number 3787:3788 text="1" suf="" isf=0 int=1
Newline 3788:3789
RBrace 3797:3798
Newline 3798:3799
If 3807:3809
LParen 3810:3811
Ident 3811:3813 text="ok"
RParen 3813:3814
LBrace 3815:3816
Newline 3816:3817
Return 3829:3835
True 3836:3840
Newline 3840:3841
RBrace 3849:3850
Newline 3850:3851
Ident 3859:3860 text="i"
PlusEq 3861:3863
Number 3864:3865 text="1" suf="" isf=0 int=1
Newline 3865:3866
RBrace 3870:3871
Newline 3871:3872
Return 3876:3882
False 3883:3888
Newline 3888:3889
RBrace 3889:3890
Newline 3890:3891
Newline 3891:3892
Ident 3892:3896 text="test"
Fn 3897:3899
Ident 3900:3927 text="fs_list_dir_missing_is_none"
LParen 3927:3928
RParen 3928:3929
LBrace 3930:3931
Newline 3931:3932
Let 3936:3939
Ident 3940:3947 text="entries"
Assign 3948:3949
Ident 3950:3958 text="list_dir"
LParen 3958:3959
Str 3959:3993 T("tests/pickle/__fs_missing_dir_zz")
RParen 3993:3994
Newline 3994:3995
If 3999:4001
LParen 4002:4003
Let 4003:4006
Ident 4007:4011 text="some"
LParen 4011:4012
Ident 4012:4015 text="_es"
RParen 4015:4016
Assign 4017:4018
Ident 4019:4026 text="entries"
RParen 4026:4027
LBrace 4028:4029
Newline 4029:4030
Ident 4038:4044 text="expect"
LParen 4044:4045
Str 4045:4057 T("unexpected")
RParen 4057:4058
Dot 4058:4059
Ident 4059:4063 text="toBe"
LParen 4063:4064
Str 4064:4092 T("missing dir listed as some")
RParen 4092:4093
Newline 4093:4094
RBrace 4098:4099
Else 4100:4104
LBrace 4105:4106
Newline 4106:4107
Ident 4115:4121 text="expect"
LParen 4121:4122
True 4122:4126
RParen 4126:4127
Dot 4127:4128
Ident 4128:4132 text="toBe"
LParen 4132:4133
True 4133:4137
RParen 4137:4138
Newline 4138:4139
RBrace 4143:4144
Newline 4144:4145
RBrace 4145:4146
Newline 4146:4147
Newline 4147:4148
Ident 4148:4152 text="test"
Fn 4153:4155
Ident 4156:4176 text="fs_bytes_bridge_read"
LParen 4176:4177
RParen 4177:4178
LBrace 4179:4180
Newline 4180:4181
Newline 4255:4256
Let 4260:4263
Ident 4264:4268 text="path"
Assign 4269:4270
Str 4271:4300 T("tests/pickle/__fs_bytes.tmp")
Newline 4300:4301
Ident 4305:4311 text="expect"
LParen 4311:4312
Ident 4312:4322 text="write_file"
LParen 4322:4323
Ident 4323:4327 text="path"
Comma 4327:4328
Str 4329:4333 T("ab")
RParen 4333:4334
RParen 4334:4335
Dot 4335:4336
Ident 4336:4340 text="toBe"
LParen 4340:4341
True 4341:4345
RParen 4345:4346
Newline 4346:4347
Let 4351:4354
Ident 4355:4359 text="data"
Assign 4360:4361
Ident 4362:4371 text="read_file"
LParen 4371:4372
Ident 4372:4376 text="path"
RParen 4376:4377
Newline 4377:4378
If 4382:4384
LParen 4385:4386
Let 4386:4389
Ident 4390:4394 text="some"
LParen 4394:4395
Ident 4395:4398 text="txt"
RParen 4398:4399
Assign 4400:4401
Ident 4402:4406 text="data"
RParen 4406:4407
LBrace 4408:4409
Newline 4409:4410
Let 4418:4421
Ident 4422:4424 text="bs"
Assign 4425:4426
Ident 4427:4432 text="bytes"
LParen 4432:4433
Ident 4433:4436 text="txt"
RParen 4436:4437
Newline 4437:4438
Ident 4446:4452 text="expect"
LParen 4452:4453
Ident 4453:4456 text="len"
LParen 4456:4457
Ident 4457:4459 text="bs"
RParen 4459:4460
RParen 4460:4461
Dot 4461:4462
Ident 4462:4466 text="toBe"
LParen 4466:4467
Number 4467:4468 text="2" suf="" isf=0 int=2
RParen 4468:4469
Newline 4469:4470
Ident 4478:4484 text="expect"
LParen 4484:4485
Ident 4485:4487 text="bs"
LBracket 4487:4488
Number 4488:4489 text="0" suf="" isf=0 int=0
RBracket 4489:4490
EqEq 4491:4493
Number 4494:4496 text="97" suf="" isf=0 int=97
RParen 4496:4497
Dot 4497:4498
Ident 4498:4502 text="toBe"
LParen 4502:4503
True 4503:4507
RParen 4507:4508
Newline 4508:4509
Ident 4517:4523 text="expect"
LParen 4523:4524
Ident 4524:4527 text="str"
LParen 4527:4528
Ident 4528:4530 text="bs"
RParen 4530:4531
RParen 4531:4532
Dot 4532:4533
Ident 4533:4537 text="toBe"
LParen 4537:4538
Str 4538:4542 T("ab")
RParen 4542:4543
Newline 4543:4544
RBrace 4548:4549
Else 4550:4554
LBrace 4555:4556
Newline 4556:4557
Ident 4565:4571 text="expect"
LParen 4571:4572
Str 4572:4584 T("unexpected")
RParen 4584:4585
Dot 4585:4586
Ident 4586:4590 text="toBe"
LParen 4590:4591
Str 4591:4655 T("bytes bridge: read_file returned none after a successful write")
RParen 4655:4656
Newline 4656:4657
RBrace 4661:4662
Newline 4662:4663
RBrace 4663:4664
Eof 4664:4664
