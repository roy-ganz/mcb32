
use embedded_hal::digital::v2::StatefulOutputPin;

pub struct StatefulLeds<L1, L2, L3, L4, L5, L6, L7, L8> 
where L1: StatefulOutputPin, L2: StatefulOutputPin,L3: StatefulOutputPin,L4: StatefulOutputPin,
L5: StatefulOutputPin,L6: StatefulOutputPin,L7: StatefulOutputPin, L8: StatefulOutputPin
{
    l1 : L1,
    l2 :L2,
    l3 :L3,
    l4 :L4,
    l5 :L5,
    l6 :L6,
    l7 :L7,
    l8 :L8,
}


// TODO 

// basic + rotate, shift, toggle, 