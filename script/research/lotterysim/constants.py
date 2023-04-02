from decimal import Decimal as Num

N_TERM = 2
CONTROLLER_TYPE_ANALOGUE=-1
CONTROLLER_TYPE_DISCRETE=0
CONTROLLER_TYPE_TAKAHASHI=1
ERC20DRK=2.1*10**9

L = 28948022309329048855892746252171976963363056481941560715954676764349967630337.0
REWARD = 1
F_MIN = 0.0001
F_MAX = 0.9999
EPSILON = 1
EPOCH_LENGTH = 10
L_HP = Num(L)
REWARD_HP = Num(REWARD)
F_MIN_HP = Num(F_MIN)
F_MAX_HP = Num(F_MAX)
EPSILON_HP = Num(EPSILON)
SLOT = 90
ONE_YEAR = 365.25*24*60*60/SLOT
