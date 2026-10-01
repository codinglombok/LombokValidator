#!/usr/bin/env python3
"""Author vector INPUTS for LombokValidator. Expected outputs are filled by the Rust reference:
`LOMBOK_REGEN=1 cargo test --test vectors` (in rust/). Deterministic (fixed seed)."""
import json, os, random
C = []
def add(fn, inp): C.append({"fn": fn, "in": inp})

EMAILS = ["user@example.com","first.last@sub.example.co.id","a+tag@sub-domain.example.co.id","no-at-sign.com","user@","@example.com","user@example","has space@example.com",
 "a@b.co","a@b.c","a@-bad.com","a@bad-.com","a..b@example.com",".a@example.com","a.@example.com","a@b..com","a@exam_ple.com","a@example.c0m","a@example.com.","a@.example.com",
 "a@@example.com","a@b@example.com","\"quoted\"@example.com","a@[127.0.0.1]","ünï@example.com","a@ünï.com","a@例え.jp","tab\t@example.com","a@example.com\n","",
 "a"*64+"@example.com","a"*65+"@example.com","a@"+"b"*63+".com","a@"+"b"*64+".com","a@"+("b"*60+".")*4+"com","x"*300+"@example.com","a@1.2.3.4","a@localhost.localdomain","a@x-y.z-w.io","A@EXAMPLE.COM"]
# exact byte-length boundaries (RFC 5321: total 254, local 64, label 63)
d254 = "b"*63+"."+"b"*63+"."+"b"*57+".com"
EMAILS += ["a"*64+"@"+d254, "a"*64+"@"+d254+"x", "a"*64+"@"+"b"*63+"."+"b"*63+"."+"b"*58+".com", "\u00e9"*32+"@example.com", "\u00e9"*33+"@example.com", "a@"+"b"*63+".co", "a@"+"b"*63+".c"+"o"*62+"m"]
for e in EMAILS: add("is_email", e)

URLS = ["https://example.com","https://example.com/path?q=1#frag","ftp://files.example.com","not a url","://missing-scheme.com","https://","http://","a+b-c.d://host","1http://x","-http://x",
 "http:/x","http//x.com","https://user:pw@host:8080/p","mailto:a@b.co","javascript:alert(1)","HTTP://EXAMPLE.COM","https://exa mple.com","https://?q=1","https:///path","https://#f","file:///etc/passwd","https://例え.jp/パス","ht tp://x","","://","x://y"]
for u in URLS: add("is_url", u)

UUIDS = ["550e8400-e29b-41d4-a716-446655440000","550E8400-E29B-41D4-A716-446655440000","not-a-uuid","550e8400-e29b-41d4-a716","550e8400e29b41d4a716446655440000","550e8400-e29b-41d4-a716-44665544000g","550e8400-e29b-41d4-a716-4466554400000","{550e8400-e29b-41d4-a716-446655440000}","","00000000-0000-0000-0000-000000000000","550e8400-e29b-41d4-a716-446655440000\n"]
for u in UUIDS: add("is_uuid", u)

LEN = [("abc",2,10),("a",2,10),("way-too-long-value",2,10),("",None,None),("",1,None),("日本語",3,3),("日本語",4,None),("é",1,1),("e\u0301",1,1),("😀😀",2,2),("abc",None,2),("abc",3,3),("abc",0,0),("abc",5,2)]
for v,mn,mx in LEN: add("validate_length", {"value": v, "min": mn, "max": mx})

RI = [(25,0,120),(-1,0,120),(200,0,120),(0,0,0),(5,None,None),(5,6,None),(5,None,4),("9223372036854775807",0,"9223372036854775807"),("-9223372036854775808","-9223372036854775808",0),(5,10,1)]
for v,mn,mx in RI: add("validate_range_i64", {"value": v, "min": mn, "max": mx})
RF = [(7.5,0,10),(11,0,10),(-0.5,0,10),(10,0,10),(0.1,0.1,0.1),(5,None,None),(1e300,None,1e299),(-1e300,-1e299,None)]
for v,mn,mx in RF: add("validate_range_f64", {"value": v, "min": mn, "max": mx})
for v in [5,0,-5,1,"9223372036854775807","-9223372036854775808"]: add("validate_positive_i64", v)
for v in [0.01,0,-0.01,1e-300,-1e-300,5]: add("validate_positive_f64", v)

for s in ["1234","-1234","12.34","","-","0","007","+5","1e5","１２３","-0"," 5","5 "]: add("is_integer_str", s)
for s in ["12.34","-0.5","5","1.2.3","abc","","-","."," .5","5.","-.5","1e5","0.0","١٢٣","--1"]: add("is_float_str", s)

PAT = [("hello","hello"),("hello","world"),("abc123","[a-z]+[0-9]*"),("123abc","[a-z]+[0-9]*"),("12345","\\d+"),("12a45","\\d+"),("hello","^hello$"),("cat","cat|dog"),("dog","cat|dog"),("fish","cat|dog"),
 ("color","colou?r"),("colour","colou?r"),("abc","[^0-9]+"),("ab1","[^0-9]+"),("2026","\\d{4}"),("202","\\d{4}"),("123","\\d{2,}"),("12","\\d{1,3}"),("1234","\\d{1,3}"),("AB-1234","[A-Z]{2}-\\d{4}"),
 ("a.c","a\\.c"),("abc","a\\.c"),("abc","a.c"),("]","[]]"),("a-b","[a\\-b]+"),("a-z","[a-z]"),("é","[à-ü]"),("日本","..") ,("日本","."),("","") ,("","a*"),("","a+"),("a","a{0}"),("aaa","a{2,3}"),("aaaa","a{2,3}"),
 ("a{","a{"),("a{1","a{1"),("ab","a{1,b"),("x","{"),("","|"),("a","|a"),("a","a|"),("Ab","[A-Z][a-z]"),("_","\\w"),(" ","\\s"),("\t\n","\\s+"),("ab","(a)(b)"),("a","[a"),("a","[a-"),("a","\\"),("$","\\$"),("a$","a$"),("a","^a$|b"),
 ("b","^a$|b"),("aaaaaaaaaaaaaaaaaaaaaaaa","(a+)+b"),("a"*30,"a*a*a*a*b"),]
for v,p in PAT: add("matches_pattern", {"value": v, "pattern": p})
add("matches_pattern", {"value": "a"*3000+"b", "pattern": "a*a*a*a*a*a*c"})
for v,p in [("a"*1000,"a{1000}"),("a"*1001,"a{1001}"),("a"*1000,"a{1500}"),("a"*1000,"a{1,2000}"),("a"*1001,"a{1,2000}"),("a"*999,"a{1000}"),("a"*1000,"a{1000,}"),("a"*1001,"a{1000,}"),("a"*5,"a{9999}"),("a"*3,"a{1,4}")]:
    add("matches_pattern", {"value": v, "pattern": p})
add("matches_pattern", {"value": "x"*20000, "pattern": "x+"})
for v,p in PAT[:12]: add("validate_pattern", {"value": v, "pattern": p})
add("validate_pattern", {"value": "a"*3000+"b", "pattern": "a*a*a*a*a*a*c"})

for s in ["<b>bold</b> text","<script>alert(1)</script>safe","5 < 10","a<b","a<b>c","<a href=\"x\">l</a>","<<>>","<","<>","a > b","<!-- c -->t","","no tags","<b>unclosed","x</b>y","<a\nb>t","<<b>>x","日<b>本</b>語","a<b","<a><b><c>x","<<<<","<<<>"]: add("strip_html", s)
for s in ["  hello  ","\t\nx\r\n","\u00a0x\u00a0","\u3000x\u3000","","   ","x"]: add("trim", s)
for s in ["HELLO World","ÀÉÎÕÜ","İstanbul","ΣΑΣ","ǅ","straße","","ABC123"]: add("to_lowercase", s)

rng = random.Random(20260928)
PIECES = ["a","b","@",".","-"," ","http://","<",">","\"","é","日","1","0","x","://","/","?","#"]
def soup(k): return "".join(rng.choice(PIECES) for _ in range(rng.randint(0,k)))
for _ in range(200): add("is_email", soup(14))
for _ in range(150): add("is_url", soup(14))
for _ in range(200): add("strip_html", soup(16))
PP = ["a","b",".","*","+","?","{","}","{2}","{1,3}","[","]","^","$","|","\\","\\d","[a-z]","[^a]",","]
for _ in range(300):
    add("matches_pattern", {"value": "".join(rng.choice(["a","b","ab","1"," "]) for _ in range(rng.randint(0,10))), "pattern": "".join(rng.choice(PP) for _ in range(rng.randint(0,9)))})
json.dump({"suite":"lombokvalidator","version":1,"cases":C}, open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"lombokvalidator-vectors-v1.json"),"w",encoding="utf-8"), ensure_ascii=False)
print(len(C),"cases")
