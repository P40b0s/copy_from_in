import new_packet_sound from '../assets/mp3/new_packet.mp3';
import err_sound from '../assets/mp3/error.mp3';

const new_packet_notify_sound = () => 
{
    const audio = new Audio(new_packet_sound);
    audio.play();
};
const error_sound = () => 
{
    const audio = new Audio(err_sound);
    audio.play();
};
export 
{
    new_packet_notify_sound,
    error_sound

};