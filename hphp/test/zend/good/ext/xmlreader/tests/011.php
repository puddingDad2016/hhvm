<?hh 
/* $Id$ */
<<__EntryPoint>> function main(): void {
$xmlstring = '<?xml version="1.0" encoding="UTF-8"?>
<books><book>test</book></books>';

$reader = new XMLReader();
$reader->XML($xmlstring);
$reader->read();
echo $reader->readInnerXML();
echo "\n";
$reader->close();


$reader = new XMLReader();
$reader->XML($xmlstring);
$reader->read();
echo $reader->readOuterXML();
echo "\n";
$reader->close();
echo "===DONE===\n";
}
