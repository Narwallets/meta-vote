import {
  Button, 
  Flex, 
  Text, 
  useDisclosure, 
  VStack,
  TableContainer,
  Table,
  Thead,
  Tr,
  Th,
  Tbody,
  Td,
  Tag,
  Square,
  Image,
  HStack,
  Accordion,
  AccordionItem,
  AccordionButton,
  AccordionPanel,
  AccordionIcon,
  Circle,
  useBreakpointValue
} from '@chakra-ui/react';
import React, { useEffect, useState } from 'react';
import { colors } from '../../../constants/colors';
import { getAllLockingPositions, relock, unlock, withdrawAPosition } from '../../../lib/near';
import { useStore as useWallet } from "../../../stores/wallet";
import { useStore as useVoter } from "../../../stores/voter";
import { getLockinPositionStatus, POSITION_STATUS, timeLeftTo, timeRemain, yton } from '../../../lib/util';
import LockModal from './LockModal';
import InfoModal, { InfoContent } from './InfoModal';
import { ACTION_TYPE, MODAL_TEXT } from '../../../constants';
import { STATUS_CODES } from 'http';
import moment from 'moment';

type Props = {
}

const LockingPosition = (props: Props) => {

  const { wallet} = useWallet();
  const { voterData, setVoterData } = useVoter();
  const [ actionCall, setActionCall] = useState(() => ()=>{}); 
  const [ modalContent, setModalContent] = useState<InfoContent>({title: '', text:''}); 

  
  const { isOpen, onClose } = useDisclosure();
  const { isOpen : infoIsOpen,  onClose : infoOnClose, onOpen: onOpenInfoModal} = useDisclosure();

  const isDesktop = useBreakpointValue({ base: false, md: true });

  const STATUS = ['Locked', 'Unlocked', 'Unloking...'];
 
  const getVotingPositions = async ()=> {
    const newVoterData = voterData;
    newVoterData.lockingPositions = await getAllLockingPositions(wallet);
    setVoterData(newVoterData);
  }
  const getTimeRemaining = (position: any): string => {
    const timeUnlockingStartAt = moment(position.unlocking_started_at);
    const unlockingFinishedTime = timeUnlockingStartAt.add(position.locking_period, 'day');



    return getLockinPositionStatus(position) === POSITION_STATUS.UNLOKING ? timeLeftTo(unlockingFinishedTime) : getLockinPositionStatus(position) === POSITION_STATUS.UNLOCKED ? '0 days' : '-'
  }

  const unlockPosition = (idPosition: string) => {
    try {
      unlock(idPosition, wallet);
    } catch (error) {
      console.error(error);
    }
  }

  const withdrawCall =  (positionId: string) => {
    try {
      withdrawAPosition(positionId, wallet); 
    } catch (error) {
      console.error(error);
    }
  }

  const relockClicked =  (positionIndex: string, period: string, amount: string) => {
    try {
      relock(positionIndex, period, amount, wallet);
    } catch (error) {
      console.error(error);
    }
  }



  const clickedAction = (idPosition: string, type: ACTION_TYPE, period? :string, amount?: string) => {
    switch (type) {
      case ACTION_TYPE.UNLOCK:
        setModalContent(MODAL_TEXT.UNLOCK)
        setActionCall(()=>()=> unlockPosition(idPosition))
        break;
      case ACTION_TYPE.RELOCK:
        setModalContent(MODAL_TEXT.RELOCK)
        if (period && amount ){
          setActionCall(()=>()=> relockClicked(idPosition, period, amount))
        }
        break;
      case ACTION_TYPE.WITHDRAW:
        setModalContent(MODAL_TEXT.WITHDRAW)
        setActionCall(()=>()=> withdrawCall(idPosition))
        break;
    }
    onOpenInfoModal();
  }

  const getStatusTag = (position: any)=> {
    const status = getLockinPositionStatus(position); 
    switch (status) {
      case POSITION_STATUS.LOCKED:
        return ( <Tag colorScheme={'red'} variant='solid'>{STATUS[getLockinPositionStatus(position)]}</Tag> )

      case POSITION_STATUS.UNLOCKED:
        return ( <Tag colorScheme={'green'} variant='solid'>{STATUS[getLockinPositionStatus(position)]}</Tag>)

      case POSITION_STATUS.UNLOKING:
        return ( <Tag colorScheme={'yellow'} variant='solid'>{STATUS[getLockinPositionStatus(position)]}</Tag> ) 
    }
  }

  const getStatusCircle = (position: any)=> {
    const status = getLockinPositionStatus(position); 
    switch (status) {
      case POSITION_STATUS.LOCKED:
        return ( <><Circle mr={5} size={3} bg={'green'}/>{STATUS[getLockinPositionStatus(position)]}</> )

      case POSITION_STATUS.UNLOCKED:
        return ( <><Circle mr={5} size={3} bg={'red'}/>{STATUS[getLockinPositionStatus(position)]}</>)

      case POSITION_STATUS.UNLOKING:
        return (<><Circle mr={5} size={3} bg={'orange'}/>{STATUS[getLockinPositionStatus(position)]}</> ) 
    }
  }

  const getButtonbyStatus = (position: any)=> {
    const status = getLockinPositionStatus(position); 
    switch (status) {
      case POSITION_STATUS.LOCKED:
        return ( <Button colorScheme={colors.primary}  w={'100%'} onClick={()=> clickedAction(position.index, ACTION_TYPE.UNLOCK)}>Start unlock</Button> )

      case POSITION_STATUS.UNLOCKED:
        return ( <Button colorScheme={colors.primary} w={'100%'} onClick={()=> clickedAction(position.index, ACTION_TYPE.WITHDRAW)}>Withdraw</Button>)

      case POSITION_STATUS.UNLOKING:
        return ( <Button colorScheme={colors.primary} w={'100%'} onClick={()=> clickedAction(position.index, ACTION_TYPE.RELOCK, position.locking_period, position.amount)}>Relock</Button> ) 
    }
  }

  useEffect(  () =>{
    (async ()=> {
      if (wallet && wallet.isSignedIn()) {
        getVotingPositions()
      }
    })();
  },[wallet])

  return (
    <section>        
        { /* *********** DESKTOP UI ***************** */
          isDesktop && (
            <TableContainer mt={30}>
              <Table  >
                <Thead>
                  <Tr>
                    <Th color={'blackAlpha.500'} fontSize={'2xl'} isNumeric>Voting Power</Th>
                    <Th color={'blackAlpha.500'} fontSize={'2xl'}isNumeric >$META amount</Th>
                    <Th color={'blackAlpha.500'} fontSize={'2xl'} isNumeric>Autolock days</Th>
                    <Th color={'blackAlpha.500'} fontSize={'2xl'} isNumeric>Remainig time</Th>
                    <Th color={'blackAlpha.500'} fontSize={'2xl'}>Status</Th>
                    <Th color={'blackAlpha.500'} fontSize={'2xl'}>Action</Th>
                  </Tr>
                </Thead>
                <Tbody>
                  {  voterData.lockingPositions.map((position: any, index: number)=> {
                      return (
                        <Tr key={index}>
                          <Td  fontSize={'2xl'} isNumeric>{yton(position.voting_power).toFixed(4)}</Td>
                          <Td  fontSize={'2xl'} isNumeric> 
                            <HStack justify={'end'}>
                              <Text>{yton(position.amount).toFixed(4)}</Text> 
                              <Square minW="30px">
                                <Image
                                  boxSize="20px"
                                  objectFit="cover"
                                  src="/meta.svg"
                                  alt="stnear"
                                />
                              </Square>
                            </HStack>
                          </Td>
                          <Td fontSize={'2xl'} isNumeric >{position.locking_period} days</Td>
                          <Td fontSize={'2xl'} isNumeric >{getTimeRemaining(position)}</Td>                          
                          <Td fontSize={'2xl'} > 
                            <HStack>{ getStatusCircle(position) } </HStack>
                          </Td>
                          <Td>
                            { getButtonbyStatus(position)}
                          </Td>
                        </Tr>
                      )
                  })}
                </Tbody>
              </Table>
            </TableContainer>)
        }

        {   /************ MOBILE UI ******************/
          !isDesktop && (
            <>
              {
                voterData.lockingPositions.map( (position: any, index: number)=> 
                 {
                  return (
                    <Accordion  key={index} allowMultiple>
                      <AccordionItem m={2} >
                        <AccordionButton  _expanded={{bg:'white'}}  bg={{base: 'white'}}>
                          <HStack w={'100%'} justify={'space-between'} textAlign="left">
                            <HStack><Circle size={3} bg={'red'}></Circle>
                            <Text fontSize={'xl'}>{position.locking_period} days </Text></HStack>
                            <Text  bg={colors.secundary+".50"} p={2} fontSize={'xl'}>{yton(position.voting_power).toFixed(4)} </Text>
                          </HStack>
                          <AccordionIcon ml={5} fontSize={'2xl'} />
                        </AccordionButton>
                        <AccordionPanel pb={4}>
                          <VStack >
                            <HStack w={'100%'} justify={'space-between'}> 
                              <Text fontSize={'xl'}>$META amount:</Text>
                              <Text p={2} bg={colors.secundary+".50"} fontSize={'xl'}> {yton(position.amount).toFixed(4)}</Text>
                            </HStack>
                            <HStack w={'100%'} justify={'space-between'}> 
                              <Text fontSize={'xl'}>Voting Power:</Text>
                              <Text p={2} bg={colors.secundary+".50"} fontSize={'xl'}> {yton(position.voting_power).toFixed(4)}</Text>
                            </HStack>
                            <HStack w={'100%'} justify={'space-between'}> 
                              <Text fontSize={'xl'}>Autolock days</Text>
                              <Text p={2} bg={colors.secundary+".50"} fontSize={'xl'}> {position.locking_period} Days</Text>
                            </HStack>
                            <HStack w={'100%'} justify={'space-between'}> 
                              <Text fontSize={'xl'}>Remaining time</Text>
                              <Text p={2} bg={colors.secundary+".50"} fontSize={'xl'}> {getTimeRemaining(position)} Days</Text>
                            </HStack>
                            <HStack  w={'100%'} justify={'space-between'}> 
                              <Text fontSize={'xl'}>Status:</Text>
                              <Text p={2} fontSize={'xl'}> {getStatusTag(position)}</Text>
                            </HStack>
                            { getButtonbyStatus(position)}

                          </VStack>
                        </AccordionPanel>
                      </AccordionItem>
                    </Accordion>
                  )
                 }
                )
              }
            </>
          )
        }             
      <InfoModal content={modalContent}  isOpen={infoIsOpen} onClose={infoOnClose} onSubmit={actionCall} ></InfoModal>
      <LockModal vPower={voterData.votingPower} isOpen={isOpen} onClose={onClose} wallet={wallet}></LockModal>
    </section>
  );
};

export default LockingPosition;


 