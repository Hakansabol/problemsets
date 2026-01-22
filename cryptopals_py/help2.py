# Source: https://node-security.com/posts/cryptography-breaking-single-byte-xor-encryption/

# Source: https://preply.com/en/blog/300-most-common-english-words/
commonEnglishWords = 'is,be,do,go,am,he,an,up,so,no,of,to,in,on,at,by,it,he,we,my,me,us,as,or,if,was,are,had,can,use,has,see,did,get,may,say,set,put,ask,try,add,saw,got,run,eat,let,cut,way,day,man,boy,end,men,air,eye,car,sea,one,all,two,new,old,any,big,own,few,not,how,out,now,too,why,off,far,for,you,his,she,her,him,who,its,our,and,but,have,were,said,will,make,like,look,been,call,find,come,made,take,know,live,give,help,tell,came,want,show,does,must,went,read,need,move,play,keep,turn,seem,open,walk,grow,took,hear,stop,miss,talk,word,time,part,work,year,back,name,line,farm,land,home,hand,page,food,tree,city,head,life,side,feet,mile,book,idea,face,girl,list,song,each,many,some,more,long,most,good,mean,same,even,such,kind,high,left,next,hard,both,four,when,then,only,very,just,much,also,well,here,away,near,last,once,soon,with,from,into,down,over,that,they,this,what,your,them,than,it’s,would,write,could,think,spell,found,study,learn,start,might,close,begin,began,carry,watch,being,leave,water,sound,place,thing,house,point,world,plant,earth,story,paper,group,night,river,state,other,great,right,three,small,large,still,every,light,white,above,young,there,first,where,again,below,never,often,later,about,after,under,along,until,which,their,these,those,while,don’t,follow,change,should,number,people,animal,letter,mother,answer,school,father,Indian,family,little,second,enough,before,around,always,really,almost,thought,picture,America,country,example,another,through,between,without,because,sentence,children,mountain,together,different,important,sometimes,something'
commonEnglishWords = commonEnglishWords.split(',')

# Source: https://www3.nd.edu/~busiforc/handouts/cryptography/Letter%20Frequencies.html
mostCommonEnglishLettersPrimary = [a for a in 'etaoin ']
mostCommonEnglishLettersSecondary = [a for a in 'shrdlu']

# Source: https://www3.nd.edu/~busiforc/handouts/cryptography/Letter%20Frequencies.html
commonEnglishBigrams = 'th,he,in,er,an,re,nd,on,en,at,ou,ed,ha,to,or,it,is,hi,es,ng'.split(
    ',')
commonEnglishTrigrams = 'the, and , ing, her, hat, his, tha, ere, for, ent, ion, ter, was, you, ith, ver, all, wit, thi, tio'.split(
    ',')

# Returns an positive Integer score based on how English the input Buffer is


def scoreDecryptedBuffer(text: str):
    commonWordMatches = sum([text.count(a) for a in commonEnglishWords])
    commonBigramMatches = sum([text.count(a) for a in commonEnglishBigrams])
    commonTrigramMatches = sum([text.count(a) for a in commonEnglishTrigrams])
    commonLetterMatchesPrimary = sum(
        [text.count(a) for a in mostCommonEnglishLettersPrimary])
    commonLetterMatchesSecondary = sum(
        [text.count(a) for a in mostCommonEnglishLettersSecondary])

    return commonWordMatches + commonBigramMatches + commonTrigramMatches + commonLetterMatchesPrimary * 2 + commonLetterMatchesSecondary
