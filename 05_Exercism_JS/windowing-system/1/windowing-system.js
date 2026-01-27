// @ts-check

/**
 * Implement the classes etc. that are needed to solve the
 * exercise in this file. Do not forget to export the entities
 * you defined so they are available for the tests.
 */
export class Size{
  constructor(w=80,h=60){
    this.width=w;
    this.height=h;
  }
  resize(w,h){
    this.width=w;
    this.height=h;
  }
}

export class Position{
  constructor(x=0,y=0){this.x=x;this.y=y;}
  move(a,b){this.x=a;this.y=b;}
}

export class ProgramWindow {
  constructor(){
    this.screenSize=new Size(800,600);
    this.size=new Size();
    this.position =new Position();
  }
  resize(size){
    
    if(size.height<1){size.height=1;}
    if(size.width<1){size.width=1;}
     console.log(this.size)
    if(this.screenSize.height<this.position.y+size.height){size.height=this.screenSize.height-this.position.y}
    if(this.screenSize.width<this.position.x+size.width){size.width=this.screenSize.width-this.position.x}
    this.size.resize(size.width,size.height);
   console.log(this.size,this.screenSize.width-this.position.x)
  }
  move(position){
    
    if(position.y<0){position.y=0;}
    if(position.x<0){position.x=0;}
    
    if(position.y+this.size.height>this.screenSize.height){
      position.y=this.screenSize.height - this.size.height;
    }
    if(position.x+this.size.width>this.screenSize.width){
      position.x=this.screenSize.width - this.size.width;
    }
    
    this.position.move(position.x,position.y);
    
    
  }
}

export function changeWindow(obj){
  obj.position.move(100,150);
  obj.size.resize(400,300);
  return obj;
}